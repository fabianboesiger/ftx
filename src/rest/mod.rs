//! This module is used to interact with the REST API.

mod error;
mod model;
#[cfg(test)]
pub(crate) mod tests;

use boolinator::Boolinator;
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
use std::{
    ops::Not,
    time::{SystemTime, UNIX_EPOCH},
};

macro_rules! deprecate_msg {
    () => {
        "This function is deprecated. Please use Rest::request instead."
    };
}
#[derive(Debug, Clone)]
pub struct Rest {
    secret: Option<String>,
    client: Client,
    subaccount: Option<String>,
    endpoint: Endpoint,
}

impl Rest {
    // TODO: this should return Result<> if it can fail
    pub fn new(
        Options {
            endpoint,
            key,
            secret,
            subaccount,
        }: Options,
    ) -> Self {
        // Set default headers.
        let headers = [
            (&key, endpoint.key_header()),
            (&subaccount, endpoint.subaccount_header()),
        ]
        .iter()
        .flat_map(|(hdr_val, hdr_ident)| hdr_val.as_ref().map(|v| (v, hdr_ident)))
        .map(|(hdr_val, hdr_key)| {
            (
                HeaderName::from_str(hdr_key).unwrap(),
                HeaderValue::from_str(hdr_val).unwrap(),
            )
        })
        .collect();

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

    pub async fn request<R: Request>(&self, req: R) -> Result<R::Response> {
        let params = matches!(R::METHOD, Method::GET).as_some(serde_qs::to_string(&req)?);
        let body = matches!(R::METHOD, Method::GET)
            .not()
            .as_some(serde_json::to_string(&req)?);

        let mut path = req.path().into_owned();
        if let Some(params) = params {
            if !params.is_empty() {
                path.push('?');
                path.push_str(&params);
            }
        }
        #[cfg(feature = "optimized-access")]
        let url = if R::OPTIMIZED_ACCESS_SUPPORTED {
            format!("{}{}", self.endpoint.optimized_access_rest(), path)
        }
        else {
            format!("{}{}", self.endpoint.rest(), path)
        };
        #[cfg(not(feature = "optimized-access"))]
        let url = format!("{}{}", self.endpoint.rest(), path);

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

        log::trace!("timestamp: {}", timestamp);
        log::trace!("method: {}", R::METHOD);
        log::trace!("path: {}", path);
        log::trace!("body: {:?}", body);

        let headers: HeaderMap = IntoIterator::into_iter([
            // Always include content_type header
            Some((
                reqwest::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            )),
            // Always include timestamp in header
            Some((
                HeaderName::from_str(self.endpoint.timestamp_header())
                    .map_err(|e| Error::Api(format!("invalid header {:?}", e)))?,
                HeaderValue::from_str(&format!("{}", timestamp))
                    .map_err(|e| Error::Api(format!("invalid header {:?}", e)))?,
            )),
            // If requires auth, include a sig
            R::AUTH.as_option().and_then(|_| {
                let secret = self.secret.as_ref().ok_or(Error::NoSecretConfigured).ok()?;

                let sign_payload = format!(
                    "{}{}/api{}{}",
                    timestamp,
                    R::METHOD,
                    path,
                    body.as_deref().unwrap_or("")
                );

                let sign = HMAC::mac(sign_payload.as_bytes(), secret.as_bytes());
                let sign = hex::encode(sign);
                Some((
                    HeaderName::from_str(self.endpoint.sign_header()).ok()?,
                    HeaderValue::from_str(&sign).ok()?,
                ))
            }),
            // If subaccount is set, include it
            self.subaccount.as_ref().and_then(|subaccount| {
                Some((
                    HeaderName::from_str(self.endpoint.subaccount_header()).ok()?,
                    HeaderValue::from_str(subaccount).ok()?,
                ))
            }),
        ])
        .flatten()
        .collect();

        let builder = self.client.request(R::METHOD, url).headers(headers);
        let builder = if let Some(body) = body {
            builder.body(body)
        } else {
            builder
        };

        let resp_body = builder.send().await?.bytes().await?;

        serde_json::from_reader(&*resp_body)
            .map(|res: SuccessResponse<R::Response>| res.result)
            .map_err(|_| {
                // try to parse the error response
                serde_json::from_reader(&*resp_body)
                    .map(|res: ErrorResponse| Error::Api(res.error))
                    // otherwise return the raw response
                    .unwrap_or_else(Into::into)
            })
            .map_err(Into::into)
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_subaccounts(&self) -> Result<<GetSubaccounts as Request>::Response> {
        self.request(GetSubaccounts {}).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn create_subaccount(
        &self,
        nickname: &str,
    ) -> Result<<CreateSubaccount as Request>::Response> {
        self.request(CreateSubaccount::new(nickname)).await
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
        self.request(GetMarkets {}).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_market(&self, market_name: &str) -> Result<<GetMarket as Request>::Response> {
        self.request(GetMarket::new(market_name)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_orderbook(
        &self,
        market_name: &str,
        depth: Option<u32>,
    ) -> Result<<GetOrderBook as Request>::Response> {
        self.request(GetOrderBook { market_name, depth }).await
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
            market_name,
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
            market_name,
            resolution,
            limit,
            start_time,
            end_time,
        })
        .await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_futures(&self) -> Result<<GetFutures as Request>::Response> {
        self.request(GetFutures {}).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_future(&self, future_name: &str) -> Result<<GetFuture as Request>::Response> {
        self.request(GetFuture::new(future_name)).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_account(&self) -> Result<<GetAccount as Request>::Response> {
        self.request(GetAccount {}).await
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
        self.request(GetCoins {}).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_positions(&self) -> Result<<GetPositions as Request>::Response> {
        self.request(GetPositions {}).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_wallet_deposit_address(
        &self,
        coin: &str,
        method: Option<&str>,
    ) -> Result<<GetWalletDepositAddress as Request>::Response> {
        self.request(GetWalletDepositAddress { coin, method }).await
    }

    #[deprecated=deprecate_msg!()]
    pub async fn get_wallet_balances(&self) -> Result<<GetWalletBalances as Request>::Response> {
        self.request(GetWalletBalances {}).await
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
            market: Some(market),
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
        // Limit orders should have price specified
        if matches!(r#type, OrderType::Limit) && price.is_none() {
            return Err(Error::PlacingLimitOrderRequiresPrice);
        }

        let req = PlaceOrder {
            market,
            side,
            price,
            r#type,
            size,
            reduce_only: reduce_only.unwrap_or_default(),
            ioc: ioc.unwrap_or_default(),
            post_only: post_only.unwrap_or_default(),
            client_id,
            reject_on_price_band: false,
        };

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
            market,
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
            client_id,
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
            client_id,
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
            market,
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
