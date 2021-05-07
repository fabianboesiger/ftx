//! This module is used to interact with the REST API.

mod model;
mod error;
#[cfg(test)]
mod tests;

pub use model::*;
pub use error::*;

use chrono::{DateTime, Utc};
use hmac_sha256::HMAC;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder, Method,
};
use rust_decimal::prelude::*;
use serde::de::DeserializeOwned;
use serde_json::{json, Map, Value};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Rest {
    secret: String,
    client: Client,
}

impl Rest {
    pub const ENDPOINT: &'static str = "https://ftx.com/api";

    pub fn new(key: String, secret: String, subaccount: Option<String>) -> Self {
        // Set default headers.
        let mut headers = HeaderMap::new();
        headers.insert("FTX-KEY", HeaderValue::from_str(&key).unwrap());
        if let Some(subaccount) = subaccount {
            headers.insert(
                "FTX-SUBACCOUNT",
                HeaderValue::from_str(&subaccount).unwrap(),
            );
        }

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { secret, client }
    }

    async fn get<T: DeserializeOwned>(&self, path: &str, params: Option<Value>) -> Result<T> {
        self.request(Method::GET, path, params, None).await
    }

    async fn post<T: DeserializeOwned>(&self, path: &str, body: Option<Value>) -> Result<T> {
        self.request(Method::POST, path, None, body).await
    }

    async fn delete<T: DeserializeOwned>(&self, path: &str, body: Option<Value>) -> Result<T> {
        self.request(Method::DELETE, path, None, body).await
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        params: Option<Value>,
        body: Option<Value>,
    ) -> Result<T> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let body = if let Some(body) = body {
            format!("{}", body)
        } else {
            String::new()
        };
        let url = format!("{}{}", Self::ENDPOINT, path);
        let sign_payload = format!("{}{}/api{}{}", timestamp, method, path, body);
        let sign = HMAC::mac(sign_payload.as_bytes(), self.secret.as_bytes());
        let sign = hex::encode(sign);
        let params = params.map(|value| {
            if let Value::Object(map) = value {
                map.into_iter()
                    .filter(|(_, v)| v != &Value::Null)
                    .collect::<Map<String, Value>>()
            } else {
                panic!("Invalid params.");
            }
        });

        log::trace!("timestamp: {}", timestamp);
        log::trace!("method: {}", method);
        log::trace!("path: {}", path);
        log::trace!("body: {}", body);

        /*
        let response: String = self.client
            .request(method, url)
            .body(body)
            .send()
            .await?
            .text()
            .await?;

        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::create("response.json").unwrap();
        file.write_all(response.as_bytes()).unwrap();

        panic!("{:?}", response);
        */

        let response: Response<T> = self
            .client
            .request(method, url)
            .query(&params)
            .header(
                "FTX-TS",
                HeaderValue::from_str(&format!("{}", timestamp)).unwrap(),
            )
            .header("FTX-SIGN", HeaderValue::from_str(&sign).unwrap())
            .body(body)
            .send()
            .await?
            .json()
            .await?;

        match response {
            Response::Result { result, .. } => Ok(result),
            Response::Error { error, .. } => Err(Error::Api(error)),
        }
    }

    pub async fn get_subaccounts(&self) -> Result<Subaccounts> {
        self.get("/subaccounts", None).await
    }

    pub async fn create_subaccount(&self, nickname: &str) -> Result<Create> {
        self.post(
            "/subaccounts",
            Some(json!({
                "nickname": nickname,
            })),
        )
        .await
    }

    pub async fn change_subaccount_name(
        &self,
        nickname: &str,
        new_nickname: &str,
    ) -> Result<ChangeName> {
        self.post(
            "/subaccounts/update_name",
            Some(json!({
                "nickname": nickname,
                "newNickname": new_nickname,
            })),
        )
        .await
    }

    pub async fn delete_subaccount(&self, nickname: &str) -> Result<Delete> {
        self.delete(
            "/subaccounts",
            Some(json!({
                "nickname": nickname,
            })),
        )
        .await
    }

    pub async fn get_subaccount_balances(&self, nickname: &str) -> Result<Balances> {
        self.get(&format!("/subaccounts/{}/balances", nickname), None)
            .await
    }

    pub async fn transfer_between_subaccounts(
        &self,
        coin: &str,
        size: Decimal,
        source: &str,
        destination: &str,
    ) -> Result<Transfer> {
        self.post(
            "/subaccounts/transfer",
            Some(json!({
                "coin": coin,
                "size": size,
                "source": source,
                "destination": destination,
            })),
        )
        .await
    }

    pub async fn get_markets(&self) -> Result<Markets> {
        self.get("/markets", None).await
    }

    pub async fn get_market(&self, market_name: &str) -> Result<Market> {
        self.get(&format!("/markets/{}", market_name), None).await
    }

    pub async fn get_orderbook(
        &self,
        market_name: &str,
        depth: Option<u32>,
    ) -> Result<Orderbook> {
        self.get(
            &format!("/markets/{}/orderbook", market_name),
            Some(json!({
                "depth": depth,
            })),
        )
        .await
    }

    pub async fn get_trades(
        &self,
        market_name: &str,
        limit: Option<u32>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Trades> {
        self.get(
            &format!("/markets/{}/trades", market_name),
            Some(json!({
                "limit": limit,
                "start_time": start_time.map(|t| t.timestamp_millis()),
                "end_time": end_time.map(|t| t.timestamp_millis()),
            })),
        )
        .await
    }

    pub async fn get_historical_prices(
        &self,
        market_name: &str,
        resolution: u32,
        limit: Option<u32>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Prices> {
        self.get(
            &format!("/markets/{}/candles", market_name),
            Some(json!({
                "resolution": resolution,
                "limit": limit,
                "start_time": start_time.map(|t| t.timestamp_millis()),
                "end_time": end_time.map(|t| t.timestamp_millis()),
            })),
        )
        .await
    }

    pub async fn get_futures(&self) -> Result<Futures> {
        self.get("/futures", None).await
    }

    pub async fn get_future(&self, future_name: &str) -> Result<Future> {
        self.get(&format!("/futures/{}", future_name), None).await
    }
}
