use super::common::{Id, OrderStatus, OrderType, Side};
use super::Request;
use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub id: Id,
    pub market: String,
    pub future: Option<String>,
    pub r#type: OrderType,
    pub side: Side,
    pub price: Option<Decimal>, // null for new market orders
    pub size: Decimal,
    pub reduce_only: Option<bool>,
    pub ioc: Option<bool>,
    pub post_only: Option<bool>,
    pub status: OrderStatus,
    pub filled_size: Option<Decimal>,
    pub remaining_size: Option<Decimal>,
    pub avg_fill_price: Option<Decimal>,
    pub liquidation: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub client_id: Option<String>,
    pub retry_until_filled: Option<bool>,
    pub trigger_price: Option<Decimal>,
    pub order_price: Option<Decimal>,
    pub triggered_at: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOpenOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
}

impl GetOpenOrdersRequest {
    pub fn all_market() -> Self {
        Self { market: None }
    }

    pub fn with_market(market: &str) -> Self {
        Self {
            market: Some(market.to_string()),
        }
    }
}

pub type GetOpenOrdersResponse = Vec<OrderInfo>;

impl Request for GetOpenOrdersRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/orders";
    const AUTH: bool = true;

    type Response = GetOpenOrdersResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub market: String,
    pub side: Side,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    pub r#type: OrderType,
    pub size: Decimal,
    pub reduce_only: bool,
    pub ioc: bool,
    pub post_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    pub reject_on_price_band: bool,
}

pub type PlaceOrderResponse = OrderInfo;

impl Request for PlaceOrderRequest {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/orders";
    const AUTH: bool = true;

    type Response = PlaceOrderResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderRequest {
    #[serde(skip_serializing)]
    pub id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

pub type ModifyOrderResponse = OrderInfo;

impl Request for ModifyOrderRequest {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/orders/{}/modify";
    const AUTH: bool = true;

    type Response = ModifyOrderResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/{}/modify", self.id))
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderRequest {
    #[serde(skip_serializing)]
    pub id: Id,
}

impl GetOrderRequest {
    pub fn new(order_id: Id) -> Self {
        Self { id: order_id }
    }
}

pub type GetOrderResponse = OrderInfo;

impl Request for GetOrderRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/orders/{}";
    const AUTH: bool = true;

    type Response = GetOrderResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/{}", self.id))
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelOrderRequest {
    #[serde(skip_serializing)]
    pub id: Id,
}

impl CancelOrderRequest {
    pub fn new(order_id: Id) -> Self {
        Self { id: order_id }
    }
}

pub type CancelOrderResponse = String;

impl Request for CancelOrderRequest {
    const METHOD: Method = Method::DELETE;
    const PATH: &'static str = "/orders/{}";
    const AUTH: bool = true;

    type Response = CancelOrderResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/{}", self.id))
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_orders_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_orders_only: Option<bool>,
}

impl CancelAllOrderRequest {
    pub fn with_market(market: &str) -> Self {
        Self {
            market: Some(market.into()),
            ..Default::default()
        }
    }
}

pub type CancelAllOrderResponse = String;

impl Request for CancelAllOrderRequest {
    const METHOD: Method = Method::DELETE;
    const PATH: &'static str = "/orders";
    const AUTH: bool = true;

    type Response = CancelAllOrderResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderByClientIdRequest {
    #[serde(skip_serializing)]
    pub client_id: String,
}

impl CancelOrderByClientIdRequest {
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.into(),
        }
    }
}

pub type CancelOrderByClientIdResponse = String;

impl Request for CancelOrderByClientIdRequest {
    const METHOD: Method = Method::DELETE;
    const PATH: &'static str = "/orders/by_client_id/{}";
    const AUTH: bool = true;

    type Response = CancelOrderByClientIdResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/by_client_id/{}", self.client_id))
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderByClientIdRequest {
    #[serde(skip_serializing)]
    pub client_id: String,
}

impl GetOrderByClientIdRequest {
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.into(),
        }
    }
}

pub type GetOrderByClientIdResponse = String;

impl Request for GetOrderByClientIdRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/orders/by_client_id/{}";
    const AUTH: bool = true;

    type Response = GetOrderByClientIdResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/by_client_id/{}", self.client_id))
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_as_timestamp"
    )]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_as_timestamp"
    )]
    pub end_time: Option<DateTime<Utc>>,
}

pub type GetOrderHistoryResponse = Vec<OrderInfo>;

impl Request for GetOrderHistoryRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/orders/history";
    const AUTH: bool = true;

    type Response = GetOrderHistoryResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTriggerOrderRequest {
    pub market: String,
    pub side: Side,
    pub size: Decimal,
    pub r#type: OrderType,
    pub trigger_price: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_until_filled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_value: Option<Decimal>,
}

pub type PlaceTriggerOrderResponse = OrderInfo;

impl Request for PlaceTriggerOrderRequest {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/conditional_orders";
    const AUTH: bool = true;

    type Response = PlaceTriggerOrderResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderByClientIdRequest {
    #[serde(skip_serializing)]
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Decimal>,
}

pub type ModifyOrderByClientIdResponse = OrderInfo;

impl Request for ModifyOrderByClientIdRequest {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/orders/by_client_id/{}/modify";
    const AUTH: bool = true;

    type Response = ModifyOrderByClientIdResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/by_client_id/{}/modify", self.client_id))
    }
}
