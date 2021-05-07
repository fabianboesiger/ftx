use chrono::{DateTime, Utc};
use serde::Deserialize;
pub use crate::rest::{Symbol, Id, Side, MarketType, Coin};
use rust_decimal::Decimal;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Channel {
    Orderbook,
    Trades,
    Ticker,
}

/*
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub channel: Channel,
    pub market: Symbol,
}
*/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub channel: Channel,
    pub market: Symbol,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    id: Id,
    price: Decimal,
    size: Decimal,
    side: Side,
    liquidation: bool,
    time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    name: Symbol,
    enabled: bool,
    price_increment: Decimal,
    size_increment: Decimal,
    #[serde(rename = "type")]
    pub market_type: MarketType,
    base_currency: Option<Coin>,
    quote_currency: Option<Coin>,
    underlying: Option<Coin>,
}