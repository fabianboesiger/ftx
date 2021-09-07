use super::common::{Id, OrderStatus, OrderType, Side};
use super::Request;
use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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
    pub reduce_only: bool,
    pub ioc: bool,
    pub post_only: bool,
    pub status: OrderStatus,
    pub filled_size: Decimal,
    pub remaining_size: Decimal,
    pub avg_fill_price: Option<Decimal>,
    pub liquidation: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub client_id: Option<String>,
}

pub type GetOpenOrdersResponse = Vec<OrderInfo>;

impl Request for GetOpenOrdersRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/orders";
    const HAS_PAYLOAD: bool = true;
    const AUTH: bool = true;

    type Response = GetOpenOrdersResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
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
    const HAS_PAYLOAD: bool = true;
    const AUTH: bool = true;

    type Response = PlaceOrderResponse;
}
