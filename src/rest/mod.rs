//! This module is used to interact with the REST API.

mod error;
mod model;
#[cfg(test)]
pub(crate) mod tests;

pub use error::*;
pub use model::*;

use crate::options::{Endpoint, Options};
use chrono::{DateTime, Utc};
use hmac_sha256::HMAC;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, ClientBuilder, Method,
};
use rust_decimal::prelude::*;
use serde_json::{from_reader, to_string, to_value};
use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! deprecate_msg {
    () => {
        "This function is deprecated. Please use Rest::request instead."
    };
}

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
        let path = req.path();
        let url = format!("{}{}", self.endpoint.rest(), path);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let (params, body) = match R::METHOD {
            Method::GET => (to_value(&req)?.as_object().cloned(), String::new()),
            _ => (None, to_string(&req)?),
        };

        log::trace!("timestamp: {}", timestamp);
        log::trace!("method: {}", R::METHOD);
        log::trace!("path: {}", path);
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
            let secret = match self.secret {
                Some(ref secret) => &**secret,
                None => {
                    return Err(Error::NoSecretConfigured);
                }
            };

            let sign_payload = format!("{}{}/api{}{}", timestamp, R::METHOD, req.path(), body);
            let sign = HMAC::mac(sign_payload.as_bytes(), secret.as_bytes());
            let sign = hex::encode(sign);
            headers.insert(
                HeaderName::from_str(&format!("{}-SIGN", self.endpoint.header_prefix())).unwrap(),
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
            .headers(headers)
            .body(body)
            .query(&params)
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

    #[deprecated=deprecate_msg!()]
    pub async fn get_subaccounts(&self) -> Result<<GetSubAccounts as Request>::Response> {
        self.request(GetSubAccounts).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn create_subaccount(
        &self,
        nickname: &str,
    ) -> Result<<CreateSubAccount as Request>::Response> {
        self.request(CreateSubAccount::new(nickname)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn change_subaccount_name(
        &self,
        nickname: &str,
        new_nickname: &str,
    ) -> Result<<ChangeSubaccountName as Request>::Response> {
        self.request(ChangeSubaccountName::new(nickname, new_nickname))
            .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn delete_subaccount(
        &self,
        nickname: &str,
    ) -> Result<<DeleteSubaccount as Request>::Response> {
        self.request(DeleteSubaccount::new(nickname)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_subaccount_balances(
        &self,
        nickname: &str,
    ) -> Result<<GetSubaccountBalances as Request>::Response> {
        self.request(GetSubaccountBalances::new(nickname)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn transfer_between_subaccounts(
        &self,
        coin: &str,
        size: Decimal,
        source: &str,
        destination: &str,
    ) -> Result<<TransferBetweenSubaccounts as Request>::Response> {
        self.request(TransferBetweenSubaccounts::new(
            coin,
            size,
            source,
            destination,
        ))
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_markets(&self) -> Result<<GetMarkets as Request>::Response> {
        self.request(GetMarkets).await
    }

    pub async fn get_market(&self, market_name: &str) -> Result<<GetMarket as Request>::Response> {
        self.request(GetMarket::new(market_name)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_orderbook(
        &self,
        market_name: &str,
        depth: Option<u32>,
    ) -> Result<<GetOrderBook as Request>::Response> {
        self.request(GetOrderBook {
            market_name: market_name.into(),
            depth,
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_trades(
        &self,
        market_name: &str,
        limit: Option<u32>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<<GetTrades as Request>::Response> {
        self.request(GetTrades {
            market_name: market_name.into(),
            limit,
            start_time,
            end_time,
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_historical_prices(
        &self,
        market_name: &str,
        resolution: u32,
        limit: Option<u32>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<<GetHistoricalPrices as Request>::Response> {
        self.request(GetHistoricalPrices {
            market_name: market_name.into(),
            resolution,
            limit,
            start_time,
            end_time,
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_futures(&self) -> Result<<GetFutures as Request>::Response> {
        self.request(GetFutures).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_future(&self, future_name: &str) -> Result<<GetFuture as Request>::Response> {
        self.request(GetFuture::new(future_name)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_account(&self) -> Result<<GetAccount as Request>::Response> {
        self.request(GetAccount).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn change_account_leverage(
        &self,
        leverage: u32,
    ) -> Result<<ChangeAccountLeverage as Request>::Response> {
        self.request(ChangeAccountLeverage::new(leverage)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_coins(&self) -> Result<<GetCoins as Request>::Response> {
        self.request(GetCoins).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_positions(&self) -> Result<<GetPositions as Request>::Response> {
        self.request(GetPositions).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_wallet_deposit_address(
        &self,
        coin: &str,
        method: Option<&str>,
    ) -> Result<<GetWalletDepositAddress as Request>::Response> {
        self.request(GetWalletDepositAddress {
            coin: coin.into(),
            method: method.map(Into::into),
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_wallet_balances(&self) -> Result<<GetWalletBalances as Request>::Response> {
        self.request(GetWalletBalances).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_wallet_deposits(
        &self,
        limit: Option<usize>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<<GetWalletDeposits as Request>::Response> {
        self.request(GetWalletDeposits {
            limit,
            start_time,
            end_time,
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_wallet_withdrawals(
        &self,
        limit: Option<usize>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<<GetWalletWithdrawals as Request>::Response> {
        self.request(GetWalletWithdrawals {
            limit,
            start_time,
            end_time,
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_open_orders(
        &self,
        market: &str,
    ) -> Result<<GetOpenOrders as Request>::Response> {
        self.request(GetOpenOrders::with_market(market)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_order_history(
        &self,
        market: &str,
        limit: Option<usize>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<<GetOrderHistory as Request>::Response> {
        self.request(GetOrderHistory {
            market: Some(market.into()),
            limit,
            start_time,
            end_time,
            ..Default::default()
        })
        .await
    }

    #[allow(clippy::too_many_arguments)]
    #[deprecated=deprecate_msg!()]
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
    ) -> Result<<PlaceOrder as Request>::Response> {
        let req = PlaceOrder {
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
    #[deprecated=deprecate_msg!()]
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
        self.request(PlaceTriggerOrder {
            market: market.into(),
            side,
            size,
            r#type,
            trigger_price,
            reduce_only,
            retry_until_filled,
            order_price,
            trail_value,
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn modify_order_by_client_id(
        &self,
        client_id: &str,
        price: Option<Decimal>,
        size: Option<Decimal>,
    ) -> Result<OrderInfo> {
        self.request(ModifyOrderByClientId {
            client_id: client_id.into(),
            price,
            size,
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn modify_order(
        &self,
        order_id: Id,
        price: Option<Decimal>,
        size: Option<Decimal>,
        client_id: Option<&str>,
    ) -> Result<<ModifyOrder as Request>::Response> {
        self.request(ModifyOrder {
            id: order_id,
            price,
            size,
            client_id: client_id.map(Into::into),
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_order(&self, order_id: Id) -> Result<<GetOrder as Request>::Response> {
        self.request(GetOrder::new(order_id)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_order_by_client_id(
        &self,
        client_id: &str,
    ) -> Result<<GetOrderByClientId as Request>::Response> {
        self.request(GetOrderByClientId::new(client_id)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn cancel_all_orders(
        &self,
        market: Option<&str>,
        side: Option<Side>,
        conditional_orders_only: Option<bool>,
        limit_orders_only: Option<bool>,
    ) -> Result<<CancelAllOrder as Request>::Response> {
        self.request(CancelAllOrder {
            market: market.map(Into::into),
            side,
            conditional_orders_only,
            limit_orders_only,
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn cancel_order(&self, order_id: Id) -> Result<<CancelOrder as Request>::Response> {
        self.request(CancelOrder::new(order_id)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn cancel_order_by_client_id(
        &self,
        client_id: &str,
    ) -> Result<<CancelOrderByClientId as Request>::Response> {
        self.request(CancelOrderByClientId::new(client_id)).await
    }
}
