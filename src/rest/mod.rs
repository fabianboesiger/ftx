//! This module is used to interact with the REST API.

mod error;
mod model;
#[cfg(test)]
pub(crate) mod tests;

pub use error::*;
pub use model::*;

use hmac_sha256::HMAC;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, ClientBuilder, Method,
};
use rust_decimal::prelude::*;
use serde_json::{from_reader, to_string};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::options::{Endpoint, Options};

pub struct Rest {
    secret: Option<String>,
    client: Client,
    subaccount: Option<String>,
    endpoint: Endpoint,
}

impl Rest {
    pub fn new(
        Options {
            endpoint,
            key,
            secret,
            subaccount,
        }: Options,
    ) -> Self {
        // Set default headers.
        let mut headers = HeaderMap::new();

        if let Some(key) = &key {
            headers.insert(
                HeaderName::from_str(&format!("{}-KEY", endpoint.header_prefix())).unwrap(),
                HeaderValue::from_str(key).unwrap(),
            );
        }

        if let Some(subaccount) = &subaccount {
            headers.insert(
                HeaderName::from_str(&format!("{}-SUBACCOUNT", endpoint.header_prefix())).unwrap(),
                HeaderValue::from_str(subaccount).unwrap(),
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
        }
    }

    pub async fn request<R>(&self, req: R) -> Result<R::Response>
    where
        R: Request,
    {
        let url = format!("{}{}", self.endpoint.rest(), req.path());
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let (params, body) = match (R::METHOD, R::HAS_PAYLOAD) {
            (Method::GET, true) => (to_string(&req)?, String::new()),
            (_, true) => (String::new(), to_string(&req)?),
            (_, false) => (String::new(), String::new()),
        };

        log::trace!("timestamp: {}", timestamp);
        log::trace!("method: {}", R::METHOD);
        log::trace!("path: {}", R::PATH);
        log::trace!("body: {}", body);

        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            HeaderName::from_str(&format!("{}-TS", self.endpoint.header_prefix())).unwrap(),
            HeaderValue::from_str(&format!("{}", timestamp)).unwrap(),
        );

        if R::AUTH {
            let sign_payload = format!("{}{}/api{}{}", timestamp, R::METHOD, req.path(), body);
            let sign = HMAC::mac(sign_payload.as_bytes(), self.secret.as_bytes());
            let sign = hex::encode(sign);
            headers.insert(
                HeaderName::from_str(&format!("{}-SIGN", self.header_prefix)).unwrap(),
                HeaderValue::from_str(&sign).unwrap(),
            );
        }

        if let Some(subaccount) = &self.subaccount {
            headers.insert(
                HeaderName::from_str(&format!("{}-SUBACCOUNT", self.endpoint.header_prefix()))
                    .unwrap(),
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
            .request(R::METHOD, url)
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

    pub async fn get_subaccounts(&self) -> Result<Vec<Subaccount>> {
        self.request(GetSubAccountsRequest {}).await
    }

    pub async fn create_subaccount(&self, nickname: &str) -> Result<CreateSubAccountResponse> {
        self.request(CreateSubAccountRequest::new(nickname)).await
    }

    // pub async fn change_subaccount_name(
    //     &self,
    //     nickname: &str,
    //     new_nickname: &str,
    // ) -> Result<ChangeName> {
    //     self.post(
    //         "/subaccounts/update_name",
    //         Some(json!({
    //             "nickname": nickname,
    //             "newNickname": new_nickname,
    //         })),
    //     )
    //     .await
    // }

    // pub async fn delete_subaccount(&self, nickname: &str) -> Result<Delete> {
    //     self.delete(
    //         "/subaccounts",
    //         Some(json!({
    //             "nickname": nickname,
    //         })),
    //     )
    //     .await
    // }

    // pub async fn get_subaccount_balances(&self, nickname: &str) -> Result<Balances> {
    //     self.get(&format!("/subaccounts/{}/balances", nickname), None)
    //         .await
    // }

    // pub async fn transfer_between_subaccounts(
    //     &self,
    //     coin: &str,
    //     size: Decimal,
    //     source: &str,
    //     destination: &str,
    // ) -> Result<Transfer> {
    //     self.post(
    //         "/subaccounts/transfer",
    //         Some(json!({
    //             "coin": coin,
    //             "size": size,
    //             "source": source,
    //             "destination": destination,
    //         })),
    //     )
    //     .await
    // }

    // pub async fn get_markets(&self) -> Result<Markets> {
    //     self.get("/markets", None).await
    // }

    // pub async fn get_market(&self, market_name: &str) -> Result<Market> {
    //     self.get(&format!("/markets/{}", market_name), None).await
    // }

    // pub async fn get_orderbook(&self, market_name: &str, depth: Option<u32>) -> Result<Orderbook> {
    //     self.get(
    //         &format!("/markets/{}/orderbook", market_name),
    //         Some(json!({
    //             "depth": depth,
    //         })),
    //     )
    //     .await
    // }

    // pub async fn get_trades(
    //     &self,
    //     market_name: &str,
    //     limit: Option<u32>,
    //     start_time: Option<DateTime<Utc>>,
    //     end_time: Option<DateTime<Utc>>,
    // ) -> Result<Trades> {
    //     self.get(
    //         &format!("/markets/{}/trades", market_name),
    //         Some(json!({
    //             "limit": limit,
    //             "start_time": start_time.map(|t| t.timestamp()),
    //             "end_time": end_time.map(|t| t.timestamp()),
    //         })),
    //     )
    //     .await
    // }

    // pub async fn get_historical_prices(
    //     &self,
    //     market_name: &str,
    //     resolution: u32,
    //     limit: Option<u32>,
    //     start_time: Option<DateTime<Utc>>,
    //     end_time: Option<DateTime<Utc>>,
    // ) -> Result<Prices> {
    //     self.get(
    //         &format!("/markets/{}/candles", market_name),
    //         Some(json!({
    //             "resolution": resolution,
    //             "limit": limit,
    //             "start_time": start_time.map(|t| t.timestamp()),
    //             "end_time": end_time.map(|t| t.timestamp()),
    //         })),
    //     )
    //     .await
    // }

    // pub async fn get_futures(&self) -> Result<Futures> {
    //     self.get("/futures", None).await
    // }

    // pub async fn get_future(&self, future_name: &str) -> Result<Future> {
    //     self.get(&format!("/futures/{}", future_name), None).await
    // }

    // pub async fn get_account(&self) -> Result<Account> {
    //     self.get("/account", None).await
    // }

    // pub async fn get_positions(&self) -> Result<Positions> {
    //     self.get("/positions", None).await
    // }

    pub async fn change_account_leverage(&self, leverage: i32) -> Result<ChangeLeverage> {
        self.post("/account/leverage", Some(json!({ "leverage": leverage })))
            .await
    }

    pub async fn get_coins(&self) -> Result<Vec<CoinInfo>> {
        self.get("/wallet/coins", None).await
    }

    // pub async fn get_wallet_deposit_address(
    //     &self,
    //     coin: &str,
    //     method: Option<&str>,
    // ) -> Result<WalletDepositAddress> {
    //     self.get(
    //         &format!(
    //             "/wallet/deposit_address/{}{}",
    //             coin,
    //             if let Some(method) = method {
    //                 format!("?method={}", method)
    //             } else {
    //                 "".to_string()
    //             }
    //         ),
    //         None,
    //     )
    //     .await
    // }

    // pub async fn get_wallet_balances(&self) -> Result<Vec<WalletBalance>> {
    //     self.get("/wallet/balances", None).await
    // }

    // pub async fn get_wallet_deposits(
    //     &self,
    //     limit: Option<usize>,
    //     start_time: Option<DateTime<Utc>>,
    //     end_time: Option<DateTime<Utc>>,
    // ) -> Result<Vec<WalletDeposit>> {
    //     let mut params = vec![];
    //     if let Some(limit) = limit {
    //         params.push(format!("limit={}", limit));
    //     }
    //     if let Some(start_time) = start_time {
    //         params.push(format!("start_time={}", start_time));
    //     }
    //     if let Some(end_time) = end_time {
    //         params.push(format!("end_time={}", end_time));
    //     }

    //     self.get(
    //         &format!(
    //             "/wallet/deposits{}{}",
    //             if params.is_empty() { "" } else { "?" },
    //             params.join("&")
    //         ),
    //         None,
    //     )
    //     .await
    // }

    pub async fn get_wallet_withdrawals(
        &self,
        limit: Option<usize>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<WalletWithdrawal>> {
        let mut params = vec![];
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(start_time) = start_time {
            params.push(format!("start_time={}", start_time.timestamp()));
        }
        if let Some(end_time) = end_time {
            params.push(format!("end_time={}", end_time.timestamp()));
        }

        self.get(
            &format!(
                "/wallet/withdrawals{}{}",
                if params.is_empty() { "" } else { "?" },
                params.join("&")
            ),
            None,
        )
        .await
    }

    pub async fn get_open_orders(&self, market: &str) -> Result<Vec<OrderInfo>> {
        self.request(GetOpenOrdersRequest::with_market(market))
            .await
    }

    // pub async fn get_order_history(
    //     &self,
    //     market: &str,
    //     limit: Option<usize>,
    //     start_time: Option<DateTime<Utc>>,
    //     end_time: Option<DateTime<Utc>>,
    // ) -> Result<Vec<OrderInfo>> {
    //     let mut params = vec![format!("market={}", market)];
    //     if let Some(limit) = limit {
    //         params.push(format!("limit={}", limit));
    //     }
    //     if let Some(start_time) = start_time {
    //         params.push(format!("start_time={}", start_time));
    //     }
    //     if let Some(end_time) = end_time {
    //         params.push(format!("end_time={}", end_time));
    //     }

    //     self.get(&format!("/orders/history?{}", params.join("&")), None)
    //         .await
    // }

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
        let req = PlaceOrderRequest {
            market: market.to_string(),
            side,
            price,
            r#type,
            size,
            reduce_only: reduce_only.unwrap_or_default(),
            ioc: ioc.unwrap_or_default(),
            post_only: post_only.unwrap_or_default(),
            client_id: client_id.map(ToString::to_string),
            reject_on_price_band: false,
        };

        // Limit orders should have price specified
        if let OrderType::Limit = r#type {
            if price.is_none() {
                return Err(Error::PlacingLimitOrderRequiresPrice);
            }
        }

        self.request(req).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn place_trigger_order(
        &self,
        market: &str,
        side: Side,
        size: Decimal,
        r#type: OrderType,
        trigger_price: Decimal,
        reduce_only: Option<bool>,
        retry_until_filled: Option<bool>,
        order_price: Option<Decimal>,
        trail_value: Option<Decimal>,
    ) -> Result<OrderInfo> {
        self.post(
            "/conditional_orders",
            Some(json!({
                "market": market,
                "side": side,
                "size": size,
                "type": r#type,
                "reduceOnly": reduce_only.unwrap_or(false),
                "retryUntilFilled": retry_until_filled.unwrap_or(true),
                "triggerPrice": trigger_price,
                "orderPrice": order_price,
                "trailValue": trail_value,
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

    // pub async fn modify_order(
    //     &self,
    //     order_id: Id,
    //     price: Option<Decimal>,
    //     size: Option<Decimal>,
    //     client_id: Option<&str>,
    // ) -> Result<OrderInfo> {
    //     self.post(
    //         format!("/orders/{}/modify", order_id).as_str(),
    //         Some(json!({
    //             "price": price,
    //             "size": size,
    //             "clientId": client_id,
    //         })),
    //     )
    //     .await
    // }

    // pub async fn get_order(&self, order_id: Id) -> Result<OrderInfo> {
    //     self.get(&format!("/orders/{}", order_id), None).await
    // }

    // pub async fn get_order_by_client_id(&self, client_id: &str) -> Result<OrderInfo> {
    //     self.get(&format!("/orders/by_client_id/{}", client_id), None)
    //         .await
    // }

    // pub async fn cancel_all_orders(
    //     &self,
    //     market: Option<&str>,
    //     side: Option<Side>,
    //     conditional_orders_only: Option<bool>,
    //     limit_orders_only: Option<bool>,
    // ) -> Result<String> {
    //     let mut payload = Map::new();
    //     if let Some(market) = market {
    //         payload.insert("market".to_string(), Value::String(market.to_string()));
    //     }

    //     if let Some(side) = side {
    //         payload.insert("side".to_string(), Value::String(to_string(&side).unwrap()));
    //     }

    //     payload.insert(
    //         "conditionalOrdersOnly".to_string(),
    //         Value::Bool(conditional_orders_only.unwrap_or(false)),
    //     );
    //     payload.insert(
    //         "limitOrdersOnly".to_string(),
    //         Value::Bool(limit_orders_only.unwrap_or(false)),
    //     );

    //     self.delete("/orders", Some(Value::Object(payload))).await
    // }

    // pub async fn cancel_order(&self, order_id: Id) -> Result<String> {
    //     self.delete(&format!("/orders/{}", order_id), None).await
    // }

    // pub async fn cancel_order_by_client_id(&self, client_id: &str) -> Result<String> {
    //     self.delete(&dbg!(format!("/orders/by_client_id/{}", client_id)), None)
    //         .await
    // }
}
