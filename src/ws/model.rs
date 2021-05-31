pub use crate::rest::{Coin, Id, MarketType, Side, Symbol};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_with::{serde_as, TimestampSecondsWithFrac};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Channel {
    Orderbook(Symbol),
    Trades(Symbol),
    Ticker(Symbol),
    Fills,
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
    pub market: Option<Symbol>,
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
    Pong,
    // Unsubscribed, // May need this in the future
    // Info,         // May need this in the future
}

/// Represents the response received from FTX, and is used for
/// deserialization
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ResponseData {
    Trades(Vec<Trade>),
    OrderbookData(OrderbookData),
    Fill(Fill),
}

/// Represents the data we return to the user
#[derive(Debug)]
pub enum Data {
    Trade(Trade),
    OrderbookData(OrderbookData),
    Fill(Fill),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: Id,
    pub price: Decimal,
    pub size: Decimal,
    pub side: Side,
    pub liquidation: bool,
    pub time: DateTime<Utc>, // API returns "2021-05-23T05:24:24.315884+00:00"
}

/// Order book data received from FTX which is used for initializing and updating
/// the OrderBook struct
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookData {
    pub action: OrderbookAction,
    // Note that bids and asks are returned in 'best' order,
    // i.e. highest to lowest bids, lowest to highest asks
    pub bids: Vec<(Decimal, Decimal)>,
    pub asks: Vec<(Decimal, Decimal)>,
    pub checksum: u32,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    pub time: DateTime<Utc>, // API returns 1621740952.5079553
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OrderbookAction {
    /// Initial snapshot of the orderbook
    Partial,
    /// Updates to the orderbook
    Update,
}

/// Represents the current state of the orderbook, guaranteed to be accurate
/// up to the best 100 bids and best 100 asks since the latest update.
/// Supports efficient insertions, updates, and deletions via a BTreeMap.
#[derive(Debug)]
pub struct Orderbook {
    pub symbol: Symbol,
    pub bids: BTreeMap<Decimal, Decimal>,
    pub asks: BTreeMap<Decimal, Decimal>,
}
impl Orderbook {
    pub fn new(symbol: Symbol) -> Orderbook {
        Orderbook {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn update(&mut self, data: &OrderbookData) {
        match data.action {
            OrderbookAction::Partial => {
                for bid in &data.bids {
                    self.bids.insert(bid.0, bid.1);
                }
                for ask in &data.asks {
                    self.asks.insert(ask.0, ask.1);
                }
            }
            OrderbookAction::Update => {
                for bid in &data.bids {
                    if bid.1 == Decimal::from(0) {
                        self.bids.remove(&bid.0);
                    } else {
                        self.bids.insert(bid.0, bid.1);
                    }
                }
                for ask in &data.asks {
                    if ask.1 == Decimal::from(0) {
                        self.asks.remove(&ask.0);
                    } else {
                        self.asks.insert(ask.0, ask.1);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub id: u64,
    pub market: Symbol,
    pub future: Option<Symbol>,
    pub base_currency: Option<Coin>,
    pub quote_currency: Option<Coin>,
    pub r#type: String, // e.g. "order"
    pub side: Side,
    pub price: Decimal,
    pub size: Decimal,
    pub order_id: u64,
    pub trade_id: u64,
    pub time: DateTime<Utc>,
    pub fee: Decimal,
    pub fee_rate: Decimal,
    pub fee_currency: Coin,
    pub liquidity: Liquidity,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Liquidity {
    Maker,
    Taker,
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
