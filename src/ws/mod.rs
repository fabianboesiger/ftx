//! This module is used to interact with the Websocket API.

mod model;
#[cfg(test)]
mod tests;

pub use model::*;

use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::{self, Message}};
use futures_util::{StreamExt, SinkExt};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use hmac_sha256::HMAC;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Tungstenite(tungstenite::Error),
    Serde(serde_json::Error),
    Terminated,
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
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>
}

impl Ws {
    pub const ENDPOINT: &'static str = "wss://ftx.com/ws";

    pub async fn connect(key: String, secret: String) -> Result<Self> {
        let (mut stream, _) = connect_async(Self::ENDPOINT).await?;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let sign_payload = format!("{}websocket_login", timestamp);
        let sign = HMAC::mac(sign_payload.as_bytes(), secret.as_bytes());
        let sign = hex::encode(sign);

        stream.send(Message::Text(json!({
            "op": "login",
            "args": {
                "key": key,
                "sign": sign,
                "time": timestamp as u64,
            }
        }).to_string())).await?;

        Ok(Self {
            stream
        })
    }

    async fn ping(&mut self) -> Result<()> {
        self.stream.send(Message::Text(json!({
            "op": "ping",
        }).to_string())).await?;

        Ok(())
    }

    pub async fn subscribe(&mut self, channel: Channel, market: &str) -> Result<()> {
        self.stream.send(Message::Text(json!({
            "op": "subscribe",
            "channel": match channel {
                Channel::Orderbook => "orderbook",
                Channel::Trades => "trades",
                Channel::Ticker => "ticker",
            },
            "market": market,
        }).to_string())).await?;
    
        Ok(())
    }

    pub async fn next(&mut self) -> Result<Response> {
        if let Some(msg) = self.stream.next().await {
            let msg = msg?;
            if let Message::Text(text) = msg {
                println!("{}", text);
                return Ok(serde_json::from_str(&text)?);
            }
        }
        
        Err(Error::Terminated)
    }
}
