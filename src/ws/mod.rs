//! This module is used to interact with the Websocket API.

mod model;
#[cfg(test)]
mod tests;

pub use model::*;

use futures_util::{SinkExt, StreamExt};
use hmac_sha256::HMAC;
use serde_json::json;
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, Message},
    MaybeTlsStream, WebSocketStream,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Tungstenite(tungstenite::Error),
    Serde(serde_json::Error),
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Error {
        Error::Tungstenite(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Serde(err)
    }
}

pub struct Ws {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    buf: VecDeque<Data>,
}

impl Ws {
    pub const ENDPOINT: &'static str = "wss://ftx.com/ws";
    pub const ENDPOINT_US: &'static str = "wss://ftx.us/ws";

    async fn connect_with_endpoint(
        endpoint: &str,
        key: String,
        secret: String,
        subaccount: Option<String>
    ) -> Result<Self> {
        let (mut stream, _) = connect_async(endpoint).await?;

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
                        "subaccount": subaccount,
                    }
                })
                .to_string(),
            ))
            .await?;

        Ok(Self {
            stream,
            buf: VecDeque::new(),
        })
    }

    pub async fn connect(key: String, secret: String, subaccount: Option<String>) -> Result<Self> {
        Self::connect_with_endpoint(Self::ENDPOINT, key, secret, subaccount).await
    }

    pub async fn connect_us(key: String, secret: String, subaccount: Option<String>) -> Result<Self> {
        Self::connect_with_endpoint(Self::ENDPOINT_US, key, secret, subaccount).await
    }

    /*
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
    */

    pub async fn subscribe(&mut self, channels: Vec<Channel>) -> Result<()> {
        for channel in channels {
            let (channel, symbol) = match channel {
                Channel::Orderbook(symbol) => ("orderbook", symbol),
                Channel::Trades(symbol) => ("trades", symbol),
                Channel::Ticker(symbol) => ("ticker", symbol),
            };

            self.stream
                .send(Message::Text(
                    json!({
                        "op": "subscribe",
                        "channel": channel,
                        "market": symbol,
                    })
                    .to_string(),
                ))
                .await?;

            match self.next_internal().await? {
                Some(Response {
                    r#type: Type::Subscribed,
                    ..
                }) => {}
                _ => panic!("Subscription confirmation expected."),
            }
        }

        Ok(())
    }

    async fn next_internal(&mut self) -> Result<Option<Response>> {
        if let Some(msg) = self.stream.next().await {
            let msg = msg?;
            if let Message::Text(text) = msg {
                return Ok(Some(serde_json::from_str(&text)?));
            }
        }

        Ok(None)
    }

    pub async fn next(&mut self) -> Result<Option<Data>> {
        // If buffer contains data, we can directly return it.
        if let Some(data) = self.buf.pop_front() {
            return Ok(Some(data));
        }

        // Fetch new data if buffer is empty.
        while let Some(response) = self.next_internal().await? {
            if let Some(data) = response.data {
                for data in data {
                    self.buf.push_back(data);
                }
            }

            if let Some(data) = self.buf.pop_front() {
                return Ok(Some(data));
            }
        }

        Ok(None)
    }
}
