use super::common::{Coin, Id, MarketType, Side, Symbol};
use super::Request;
use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    #[serde(rename = "type")]
    pub market_type: MarketType,
    pub name: Symbol,
    pub underlying: Option<Coin>,
    pub base_currency: Option<Coin>,
    pub quote_currency: Option<Coin>,
    pub enabled: bool,
    pub ask: Decimal,
    pub bid: Decimal,
    pub last: Option<Decimal>,
    pub post_only: bool,
    pub price_increment: Decimal,
    pub size_increment: Decimal,
    pub restricted: bool,
    pub min_provide_size: Decimal,
    pub price: Decimal,
    pub high_leverage_fee_exempt: bool,
    pub change1h: Decimal,
    pub change24h: Decimal,
    pub change_bod: Decimal,
    pub quote_volume24h: Decimal,
    pub volume_usd24h: Decimal,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMarketsRequest;

pub type GetMarketsResponse = Vec<Market>;

impl Request for GetMarketsRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets";
    const HAS_PAYLOAD: bool = false;
    const AUTH: bool = true;

    type Response = GetMarketsResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMarketRequest {
    #[serde(skip_serializing)]
    pub market_name: String,
}

impl GetMarketRequest {
    pub fn new(market_name: &str) -> Self {
        Self {
            market_name: market_name.into(),
        }
    }
}

pub type GetMarketResponse = Market;

impl Request for GetMarketRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets/{}";
    const HAS_PAYLOAD: bool = false;
    const AUTH: bool = true;

    type Response = GetMarketResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/markets/{}", self.market_name))
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    pub asks: Vec<(Decimal, Decimal)>,
    pub bids: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderBookRequest {
    #[serde(skip_serializing)]
    pub market_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

impl GetOrderBookRequest {
    pub fn new(market_name: &str) -> Self {
        Self {
            market_name: market_name.into(),
            ..Default::default()
        }
    }
}

pub type GetOrderBookResponse = Orderbook;

impl Request for GetOrderBookRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets/{}/orderbook";
    const HAS_PAYLOAD: bool = true;
    const AUTH: bool = true;

    type Response = GetOrderBookResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/markets/{}/orderbook", self.market_name))
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: Id,
    pub liquidation: bool,
    pub price: Decimal,
    pub side: Side,
    pub size: Decimal,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTradesRequest {
    #[serde(skip_serializing)]
    pub market_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
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

impl GetTradesRequest {
    pub fn new(market_name: &str) -> Self {
        Self {
            market_name: market_name.into(),
            ..Default::default()
        }
    }
}

pub type GetTradesResponse = Vec<Trade>;

impl Request for GetTradesRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets/{}/trades";
    const HAS_PAYLOAD: bool = true;
    const AUTH: bool = true;

    type Response = GetTradesResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/markets/{}/trades", self.market_name))
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub open: Decimal,
    pub volume: Decimal,
    pub start_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoricalPricesRequest {
    #[serde(skip_serializing)]
    pub market_name: String,
    pub resolution: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
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

impl GetHistoricalPricesRequest {
    pub fn new(market_name: &str, resolution: u32) -> Self {
        Self {
            market_name: market_name.into(),
            resolution,
            ..Default::default()
        }
    }
}

pub type GetHistoricalPricesResponse = Vec<Price>;

impl Request for GetHistoricalPricesRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets/{}/candles";
    const HAS_PAYLOAD: bool = true;
    const AUTH: bool = true;

    type Response = GetHistoricalPricesResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/markets/{}/candles", self.market_name))
    }
}
