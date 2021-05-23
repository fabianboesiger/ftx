pub use crate::rest::{Coin, Id, MarketType, Side, Symbol};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Channel {
    Orderbook(Symbol),
    Trades(Symbol),
    Ticker(Symbol),
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
    pub market: Symbol,
    pub r#type: Type,
    pub data: Option<ResponseData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Subscribed,
    Update,
    Error,
    Partial,
    // Unsubscribed, // May need this in the future
    // Info,         // May need this in the future
}

/// This represents the response received from FTX, and is used for
/// deserialization
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ResponseData {
    Trades(Vec<Trade>),
    OrderBook(OrderBook),
}

/// This represents the data we return to the user
#[derive(Debug)]
pub enum Data {
    Trade(Trade),
    OrderBook(OrderBook),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: Id,
    pub price: Decimal,
    pub size: Decimal,
    pub side: Side,
    pub liquidation: bool,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub action: OrderBookAction,
    pub bids: Vec<[Decimal; 2]>,
    pub asks: Vec<[Decimal; 2]>,
    pub checksum: u32,
    pub time: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderBookAction {
    /// Initial snapshot of the orderbook
    Partial,
    /// Updates to the orderbook
    Update,
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
