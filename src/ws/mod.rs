//! This module is used to interact with the Websocket API.

mod error;
mod model;
#[cfg(test)]
mod tests;

pub use error::*;
pub use model::*;

use crate::options::Options;
use futures::{
    ready,
    task::{Context, Poll},
    Future, SinkExt, Stream, StreamExt,
};
use hmac_sha256::HMAC;
use serde_json::json;
use std::collections::VecDeque;
use std::pin::Pin;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::net::TcpStream;
use tokio::time; // 1.3.0
use tokio::time::Interval;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub struct Ws {
    channels: Vec<Channel>,
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    buf: VecDeque<(Option<Symbol>, Data)>,
    ping_timer: Interval,
    /// Whether the websocket was opened authenticated with API keys or not
    is_authenticated: bool,
}

impl Ws {
    pub const ENDPOINT: &'static str = "wss://ftx.com/ws";
    pub const ENDPOINT_US: &'static str = "wss://ftx.us/ws";

    pub async fn connect(options: Options) -> Result<Self> {
        let (mut stream, _) = connect_async(options.endpoint.ws()).await?;
        let is_authenticated = if let (Some(key), Some(secret)) = (options.key, options.secret) {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let sign_payload = format!("{}websocket_login", timestamp);
            let sign = HMAC::mac(sign_payload.as_bytes(), secret.as_bytes());
            let sign = hex::encode(sign);

            stream
                .send(Message::Text(
                    json!({
                        "op": "login",
                        "args": {
                            "key": key,
                            "sign": sign,
                            "time": timestamp as u64,
                            "subaccount": options.subaccount,
                        }
                    })
                    .to_string(),
                ))
                .await?;
            true
        } else {
            false
        };
        Ok(Self {
            channels: Vec::new(),
            stream,
            buf: VecDeque::new(),
            ping_timer: time::interval(Duration::from_secs(15)),
            is_authenticated,
        })
    }

    async fn ping(&mut self) -> Result<()> {
        self.stream
            .send(Message::Text(
                json!({
                    "op": "ping",
                })
                .to_string(),
            ))
            .await?;

        Ok(())
    }

    /// Subscribe to specified `Channel`s
    /// For FILLS the socket needs to be authenticated
    pub async fn subscribe(&mut self, channels: &[Channel]) -> Result<()> {
        for channel in channels.iter() {
            // Subscribing to fills or orders requires us to be authenticated via an API key
            if (channel == &Channel::Fills || channel == &Channel::Orders) && !self.is_authenticated
            {
                return Err(Error::SocketNotAuthenticated);
            }
            self.channels.push(channel.clone());
        }

        self.subscribe_or_unsubscribe(channels, true).await?;

        Ok(())
    }

    /// Unsubscribe from specified `Channel`s
    pub async fn unsubscribe(&mut self, channels: &[Channel]) -> Result<()> {
        // Check that the specified channels match an existing one
        for channel in channels.iter() {
            if !self.channels.contains(channel) {
                return Err(Error::NotSubscribedToThisChannel(channel.clone()));
            }
        }

        self.subscribe_or_unsubscribe(channels, false).await?;

        // Unsubscribe successful, remove specified channels from self.channels
        self.channels.retain(|c| !channels.contains(c));

        Ok(())
    }

    /// Unsubscribe from all currently subscribed `Channel`s
    pub async fn unsubscribe_all(&mut self) -> Result<()> {
        let channels = self.channels.clone();
        self.unsubscribe(&channels).await?;

        self.channels.clear();

        Ok(())
    }

    async fn subscribe_or_unsubscribe(
        &mut self,
        channels: &[Channel],
        subscribe: bool,
    ) -> Result<()> {
        let op = if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        };

        'channels: for channel in channels {
            let (channel, symbol) = match channel {
                Channel::Orderbook(symbol) => ("orderbook", symbol.as_str()),
                Channel::Trades(symbol) => ("trades", symbol.as_str()),
                Channel::Ticker(symbol) => ("ticker", symbol.as_str()),
                Channel::Fills => ("fills", ""),
                Channel::Orders => ("orders", ""),
            };

            self.stream
                .send(Message::Text(
                    json!({
                        "op": op,
                        "channel": channel,
                        "market": symbol,
                    })
                    .to_string(),
                ))
                .await?;

            // Confirmation should arrive within the next 100 updates
            for _ in 0..100 {
                let response = self.next_response().await?;
                match response {
                    Response {
                        r#type: Type::Subscribed,
                        ..
                    } if subscribe => {
                        // Subscribe confirmed
                        continue 'channels;
                    }
                    Response {
                        r#type: Type::Unsubscribed,
                        ..
                    } if !subscribe => {
                        // Unsubscribe confirmed
                        continue 'channels;
                    }
                    _ => {
                        // Otherwise, continue adding contents to buffer
                        self.handle_response(response);
                    }
                }
            }

            return Err(Error::MissingSubscriptionConfirmation);
        }

        Ok(())
    }

    async fn next_response(&mut self) -> Result<Response> {
        loop {
            tokio::select! {
                _ = self.ping_timer.tick() => {
                    self.ping().await?;
                },
                Some(msg) = self.stream.next() => {
                    let msg = msg?;
                    if let Message::Text(text) = msg {
                        // println!("{}", text); // Uncomment for debugging
                        let response: Response = serde_json::from_str(&text)?;

                        // Don't return Pong responses
                        if let Response { r#type: Type::Pong, .. } = response {
                            continue;
                        }

                        return Ok(response)
                    }
                },
            }
        }
    }

    /// Helper function that takes a response and adds the contents to the buffer
    fn handle_response(&mut self, response: Response) {
        if let Some(data) = response.data {
            match data {
                ResponseData::Trades(trades) => {
                    // Trades channel returns an array of single trades.
                    // Buffer so that the user receives trades one at a time
                    for trade in trades {
                        self.buf
                            .push_back((response.market.clone(), Data::Trade(trade)));
                    }
                }
                ResponseData::OrderbookData(orderbook) => {
                    self.buf
                        .push_back((response.market, Data::OrderbookData(orderbook)));
                }
                ResponseData::Fill(fill) => {
                    self.buf.push_back((response.market, Data::Fill(fill)));
                }
                ResponseData::Ticker(ticker) => {
                    self.buf.push_back((response.market, Data::Ticker(ticker)));
                }
                ResponseData::Order(order) => {
                    self.buf.push_back((response.market, Data::Order(order)));
                }
            }
        }
    }
}

impl Stream for Ws {
    type Item = Result<(Option<Symbol>, Data)>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(data) = self.buf.pop_front() {
                return Poll::Ready(Some(Ok(data)));
            }
            let response = {
                // Fetch new response if buffer is empty.
                // safety: this is ok because the future from self.next_response() will only live in this function.
                // It won't be moved anymore.
                let mut next_response = self.next_response();
                let pinned = unsafe { Pin::new_unchecked(&mut next_response) };
                match ready!(pinned.poll(cx)) {
                    Ok(response) => response,
                    Err(e) => {
                        return Poll::Ready(Some(Err(e)));
                    }
                }
            };
            // Handle the response, possibly adding to the buffer
            self.handle_response(response);
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.buf.len(), None)
    }
}
