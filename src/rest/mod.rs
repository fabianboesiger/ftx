//! This module is used to interact with the REST API.

mod error;
mod model;
#[cfg(test)]
pub(crate) mod tests;

pub use error::*;
pub use model::*;

use chrono::{DateTime, Utc};
use hmac_sha256::HMAC;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, ClientBuilder, Method,
};
use rust_decimal::prelude::*;
use serde::de::DeserializeOwned;
use serde_json::{from_reader, json, to_string, Map, Value};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Rest {
    secret: String,
    client: Client,
    subaccount: Option<String>,
    endpoint: &'static str,
    header_prefix: &'static str,
}

impl Rest {
    pub const ENDPOINT: &'static str = "https://ftx.com/api";
    pub const ENDPOINT_US: &'static str = "https://ftx.us/api";

    fn new_with_endpoint(
        endpoint: &'static str,
        header_prefix: &'static str,
        key: String,
        secret: String,
        subaccount: Option<String>,
    ) -> Self {
        // Set default headers.
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_str(&format!("{}-KEY", header_prefix)).unwrap(),
            HeaderValue::from_str(&key).unwrap(),
        );
        if let Some(subaccount) = subaccount.to_owned() {
            headers.insert(
                HeaderName::from_str(&format!("{}-SUBACCOUNT", header_prefix)).unwrap(),
                HeaderValue::from_str(&subaccount).unwrap(),
            );
        }

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            secret,
            client,
            subaccount,
            endpoint,
            header_prefix,
        }
    }

    pub fn new(key: String, secret: String, subaccount: Option<String>) -> Self {
        Self::new_with_endpoint(Self::ENDPOINT, "FTX", key, secret, subaccount)
    }

    pub fn new_us(key: String, secret: String, subaccount: Option<String>) -> Self {
        Self::new_with_endpoint(Self::ENDPOINT_US, "FTXUS", key, secret, subaccount)
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
        let url = format!("{}{}", self.endpoint, path);
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

        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            HeaderName::from_str(&format!("{}-TS", self.header_prefix)).unwrap(),
            HeaderValue::from_str(&format!("{}", timestamp)).unwrap(),
        );
        headers.insert(
            HeaderName::from_str(&format!("{}-SIGN", self.header_prefix)).unwrap(),
            HeaderValue::from_str(&sign).unwrap(),
        );
        if let Some(subaccount) = &self.subaccount {
            headers.insert(
                HeaderName::from_str(&format!("{}-SUBACCOUNT", self.header_prefix)).unwrap(),
                HeaderValue::from_str(subaccount).unwrap(),
            );
        }

        /*
        let response: String = self
            .client
            .request(method, url)
            .query(&params)
            .headers(headers)
            .body(body)
            .send()
            .await?
            .text()
            .await?;

        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::create("response.json").unwrap();
        file.write_all(response.as_bytes()).unwrap();

        panic!("{:#?}", response);
        */

        let body = self
            .client
            .request(method, url)
            .query(&params)
            .headers(headers)
            .body(body)
            .send()
            .await?
            .bytes()
            .await?;

        match from_reader(&*body) {
            Ok(SuccessResponse { result, .. }) => Ok(result),

            Err(e) => {
                if let Ok(ErrorResponse { error, .. }) = from_reader(&*body) {
                    Err(Error::Api(error))
                } else {
                    Err(e.into())
                }
            }
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

    pub async fn get_orderbook(&self, market_name: &str, depth: Option<u32>) -> Result<Orderbook> {
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
                "start_time": start_time.map(|t| t.timestamp()),
                "end_time": end_time.map(|t| t.timestamp()),
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
                "start_time": start_time.map(|t| t.timestamp()),
                "end_time": end_time.map(|t| t.timestamp()),
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

    pub async fn get_account(&self) -> Result<Account> {
        self.get("/account", None).await
    }

    pub async fn get_positions(&self) -> Result<Positions> {
        self.get("/positions", None).await
    }

    pub async fn get_wallet_deposit_address(
        &self,
        coin: &str,
        method: Option<&str>,
    ) -> Result<WalletDepositAddress> {
        self.get(
            &format!(
                "/wallet/deposit_address/{}{}",
                coin,
                if let Some(method) = method {
                    format!("?method={}", method)
                } else {
                    "".to_string()
                }
            ),
            None,
        )
        .await
    }

    pub async fn get_wallet_balances(&self) -> Result<Vec<WalletBalance>> {
        self.get("/wallet/balances", None).await
    }

    pub async fn get_wallet_deposits(
        &self,
        limit: Option<usize>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<WalletDeposit>> {
        let mut params = vec![];
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(start_time) = start_time {
            params.push(format!("start_time={}", start_time));
        }
        if let Some(end_time) = end_time {
            params.push(format!("end_time={}", end_time));
        }

        self.get(
            &format!(
                "/wallet/deposits{}{}",
                if params.is_empty() { "" } else { "?" },
                params.join("&")
            ),
            None,
        )
        .await
    }

    pub async fn get_open_orders(&self, market: &str) -> Result<Vec<OrderInfo>> {
        self.get(&format!("/orders?market={}", market), None).await
    }

    pub async fn get_order_history(
        &self,
        market: Option<&str>,
        limit: Option<usize>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<OrderInfo>> {
        let mut params = vec![];
        if let Some(market) = market {
            params.push(format!("market={}", market));
        }
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(start_time) = start_time {
            params.push(format!("start_time={}", start_time));
        }
        if let Some(end_time) = end_time {
            params.push(format!("end_time={}", end_time));
        }

        self.get(
            &format!(
                "/orders/history{}{}",
                if params.is_empty() { "" } else { "?" },
                params.join("&")
            ),
            None,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn place_order(
        &self,
        market: &str,
        side: Side,
        price: Option<Decimal>,
        r#type: OrderType,
        size: Decimal,
        reduce_only: Option<bool>,
        ioc: Option<bool>,
        post_only: Option<bool>,
        client_id: Option<&str>,
    ) -> Result<OrderInfo> {
        // Limit orders should have price specified
        if let OrderType::Limit = r#type {
            if price.is_none() {
                return Err(Error::PlacingLimitOrderRequiresPrice);
            }
        }

        self.post(
            "/orders",
            Some(json!({
                "market": market,
                "side": side,
                // As per docs, send null for market orders
                "price": if let OrderType::Limit = r#type { price } else { None },
                "type": r#type,
                "size": size,
                "reduceOnly": reduce_only.unwrap_or(false),
                "ioc": ioc.unwrap_or(false),
                "postOnly": post_only.unwrap_or(false),
                "clientId": client_id,
            })),
        )
        .await
    }

    pub async fn modify_order(
        &self,
        order_id: Id,
        price: Option<Decimal>,
        size: Option<Decimal>,
        client_id: Option<&str>,
    ) -> Result<OrderInfo> {
        self.post(
            format!("/orders/{}/modify", order_id).as_str(),
            Some(json!({
                "price": price,
                "size": size,
                "clientId": client_id,
            })),
        )
        .await
    }

    pub async fn modify_order_by_client_id(
        &self,
        client_id: &str,
        price: Option<Decimal>,
        size: Option<Decimal>,
    ) -> Result<OrderInfo> {
        self.post(
            format!("/orders/by_client_id/{}/modify", client_id).as_str(),
            Some(json!({
                "price": price,
                "size": size,
                "clientId": client_id,
            })),
        )
        .await
    }

    pub async fn get_order(&self, order_id: Id) -> Result<OrderInfo> {
        self.get(&format!("/orders/{}", order_id), None).await
    }

    pub async fn get_order_by_client_id(&self, client_id: &str) -> Result<OrderInfo> {
        self.get(&format!("/orders/by_client_id/{}", client_id), None)
            .await
    }

    pub async fn cancel_all_orders(
        &self,
        market: Option<&str>,
        side: Option<Side>,
        conditional_orders_only: Option<bool>,
        limit_orders_only: Option<bool>,
    ) -> Result<String> {
        let mut payload = Map::new();
        if let Some(market) = market {
            payload.insert("market".to_string(), Value::String(market.to_string()));
        }

        if let Some(side) = side {
            payload.insert("side".to_string(), Value::String(to_string(&side).unwrap()));
        }

        payload.insert(
            "conditionalOrdersOnly".to_string(),
            Value::Bool(conditional_orders_only.unwrap_or(false)),
        );
        payload.insert(
            "limitOrdersOnly".to_string(),
            Value::Bool(limit_orders_only.unwrap_or(false)),
        );

        self.delete("/orders", Some(Value::Object(payload))).await
    }

    pub async fn cancel_order(&self, order_id: Id) -> Result<String> {
        self.delete(&format!("/orders/{}", order_id), None).await
    }

    pub async fn cancel_order_by_client_id(&self, client_id: &str) -> Result<String> {
        self.delete(&dbg!(format!("/orders/by_client_id/{}", client_id)), None)
            .await
    }
}
