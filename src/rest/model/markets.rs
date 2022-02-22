use super::common::{Coin, Id, MarketType, Resolution, Side, Symbol};
use super::Request;
use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    #[serde(rename = "type")]
    pub market_type: MarketType,
    pub name: Symbol,
    pub underlying: Option<Coin>,
    pub base_currency: Option<Coin>,
    pub quote_currency: Option<Coin>,
    pub enabled: bool,
    pub ask: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub last: Option<Decimal>,
    pub post_only: bool,
    pub price_increment: Decimal,
    pub size_increment: Decimal,
    pub restricted: bool,
    pub min_provide_size: Decimal,
    pub price: Option<Decimal>, // Sometimes, there is no price available?
    pub high_leverage_fee_exempt: bool,
    pub change1h: Decimal,
    pub change24h: Decimal,
    pub change_bod: Decimal,
    pub quote_volume24h: Decimal,
    pub volume_usd24h: Decimal,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMarkets {}

impl Request for GetMarkets {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets";
    const AUTH: bool = false;

    type Response = Vec<Market>;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMarket {
    #[serde(skip_serializing)]
    pub market_name: String,
}

impl GetMarket {
    pub fn new(market_name: &str) -> Self {
        Self {
            market_name: market_name.into(),
        }
    }
}

impl Request for GetMarket {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets/{}";
    const AUTH: bool = false;

    type Response = Market;

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
pub struct GetOrderBook {
    #[serde(skip_serializing)]
    pub market_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

impl GetOrderBook {
    pub fn new(market_name: &str) -> Self {
        Self {
            market_name: market_name.into(),
            ..Default::default()
        }
    }

    pub fn with_depth(market_name: &str, depth: u32) -> Self {
        Self {
            market_name: market_name.into(),
            depth: Some(depth),
        }
    }
}

impl Request for GetOrderBook {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets/{}/orderbook";
    const AUTH: bool = false;

    type Response = Orderbook;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/markets/{}/orderbook", self.market_name))
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
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
pub struct GetTrades {
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

impl GetTrades {
    pub fn new(market_name: &str) -> Self {
        Self {
            market_name: market_name.into(),
            ..Default::default()
        }
    }
    // User can specify limit, start_time, and end_time.
    // If none, use Option::None as parameter.
    pub fn new_paged(
        market_name: &str,
        limit: Option<u32>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            market_name: market_name.into(),
            limit,
            start_time,
            end_time,
        }
    }
}

impl Request for GetTrades {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets/{}/trades";
    const AUTH: bool = false;

    type Response = Vec<Trade>;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/markets/{}/trades", self.market_name))
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub open: Decimal,
    pub volume: Decimal,
    pub start_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetHistoricalPrices {
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

impl GetHistoricalPrices {
    pub fn new(market_name: &str, resolution: Resolution) -> Self {
        Self {
            market_name: market_name.into(),
            resolution: resolution.get_seconds(),
            ..Default::default()
        }
    }
    // User can specify limit, start_time, and end_time.
    // If none, use Option::None as parameter.
    pub fn new_paged(
        market_name: &str,
        resolution: Resolution,
        limit: Option<u32>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            market_name: market_name.into(),
            resolution: resolution.get_seconds(),
            limit,
            start_time,
            end_time,
        }
    }
}

impl Request for GetHistoricalPrices {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/markets/{}/candles";
    const AUTH: bool = false;

    type Response = Vec<Candle>;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/markets/{}/candles", self.market_name))
    }
}
