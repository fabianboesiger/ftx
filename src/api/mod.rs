mod model;
#[cfg(test)]
mod tests;

pub use model::*;

use std::time::{SystemTime, UNIX_EPOCH};
use serde::de::DeserializeOwned;
use serde_json::{json, Value, Map};
use hmac_sha256::HMAC;
use reqwest::{ClientBuilder, Client, Method, header::{HeaderMap, HeaderValue}};
use rust_decimal::prelude::*;
use chrono::{DateTime, Utc};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Api(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Request(err)
    }
}

pub struct Api {
    secret: String,
    client: Client,
}

impl Api {
    pub const ENDPOINT: &'static str = "https://ftx.com/api";

    pub fn new(key: String, secret: String, subaccount: Option<String>) -> Self {
        // Set default headers.
        let mut headers = HeaderMap::new();
        headers.insert("FTX-KEY", HeaderValue::from_str(&key).unwrap());
        if let Some(subaccount) = subaccount {
            headers.insert("FTX-SUBACCOUNT", HeaderValue::from_str(&subaccount).unwrap());
        }

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            secret,
            client,
        }
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

    async fn request<T: DeserializeOwned>(&self, method: Method, path: &str, params: Option<Value>, body: Option<Value>) -> Result<T> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
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
                map.into_iter().filter(|(_, v)| v != &Value::Null).collect::<Map<String, Value>>()
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
        
        let response: Response<T> = self.client
            .request(method, url)
            .query(&params)
            .header("FTX-TS", HeaderValue::from_str(&format!("{}", timestamp)).unwrap())
            .header("FTX-SIGN", HeaderValue::from_str(&sign).unwrap())
            .body(body)
            .send()
            .await?
            .json()
            .await?;

        match response {
            Response::Result {result , .. } => Ok(result),
            Response::Error {error , .. } => Err(Error::Api(error)),
        }
    }

    
    pub async fn get_subaccounts(&self) -> Result<subaccounts::Subaccounts> {
        self.get(
            "/subaccounts",
            None,
        ).await
    }

    pub async fn create_subaccount(&self, nickname: &str) -> Result<subaccounts::Create> {
        self.post(
            "/subaccounts", 
            Some(json!({
                "nickname": nickname,
            }))
        ).await
    }

    pub async fn change_subaccount_name(&self, nickname: &str, new_nickname: &str) -> Result<subaccounts::ChangeName> {
        self.post(
            "/subaccounts/update_name", 
            Some(json!({
                "nickname": nickname,
                "newNickname": new_nickname,
            }))
        ).await
    }

    pub async fn delete_subaccount(&self, nickname: &str) -> Result<subaccounts::Delete> {
        self.delete(
            "/subaccounts", 
            Some(json!({
                "nickname": nickname,
            }))
        ).await
    }

    pub async fn get_subaccount_balances(&self, nickname: &str) -> Result<subaccounts::Balances> {
        self.get(
            &format!("/subaccounts/{}/balances", nickname), 
            None,
        ).await
    }

    pub async fn transfer_between_subaccounts(&self, coin: &str, size: Decimal, source: &str, destination: &str) -> Result<subaccounts::Transfer> {
        self.post(
            "/subaccounts/transfer", 
            Some(json!({
                "coin": coin,
                "size": size,
                "source": source,
                "destination": destination,
            }))
        ).await
    }

    pub async fn get_markets(&self) -> Result<markets::Markets> {
        self.get(
            "/markets", 
            None,
        ).await
    }

    pub async fn get_market(&self, market_name: &str) -> Result<markets::Market> {
        self.get(
            &format!("/markets/{}", market_name), 
            None,
        ).await
    }

    pub async fn get_orderbook(&self, market_name: &str, depth: Option<u32>) -> Result<markets::Orderbook> {
        self.get(
            &format!("/markets/{}/orderbook", market_name), 
            Some(json!({
                "depth": depth,
            }))
        ).await
    }

    pub async fn get_trades(&self, market_name: &str, limit: Option<u32>, start_time: Option<DateTime<Utc>>, end_time: Option<DateTime<Utc>>) -> Result<markets::Trades> {
        self.get(
            &format!("/markets/{}/trades", market_name), 
            Some(json!({
                "limit": limit,
                "start_time": start_time.map(|t| t.timestamp_millis()),
                "end_time": end_time.map(|t| t.timestamp_millis()),
            }))
        ).await
    }

    pub async fn get_account(&self) -> Result<accounts::Account> {
        self.get("/account", None).await
    }

    pub async fn get_positions(&self) -> Result<Vec<accounts::Position>> {
        self.get("/positions", None).await
    }
}
