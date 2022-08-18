pub use crate::rest::{Coin, Id, MarketType, OrderInfo, Side, Symbol, Trade};
use chrono::{DateTime, Utc};
use crc32fast::Hasher;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSecondsWithFrac};
use std::{collections::BTreeMap, ops::Not};

use super::Error;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Channel {
    Orderbook(Symbol),
    Trades(Symbol),
    Ticker(Symbol),
    Fills,
    Orders,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub market: Option<Symbol>,
    pub r#type: Type,
    pub data: Option<ResponseData>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Subscribed,
    Unsubscribed,
    Update,
    Error,
    Partial,
    Pong,
    Info,
}

/// Represents the response received from FTX, and is used for
/// deserialization
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ResponseData {
    Ticker(Ticker),
    Trades(Vec<Trade>),
    OrderbookData(OrderbookData),
    Fill(Fill),
    Order(OrderInfo),
}

/// Represents the data we return to the user
#[derive(Clone, Debug, Serialize)]
pub enum Data {
    Ticker(Ticker),
    Trade(Trade),
    OrderbookData(OrderbookData),
    Fill(Fill),
    Order(OrderInfo),
}

#[serde_as]
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub bid: Decimal,
    pub ask: Decimal,
    pub bid_size: Decimal,
    pub ask_size: Decimal,
    pub last: Decimal,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    pub time: DateTime<Utc>,
}

/// Order book data received from FTX which is used for initializing and updating
/// the OrderBook struct
#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookData {
    pub action: OrderbookAction,
    // Note that bids and asks are returned in 'best' order,
    // i.e. highest to lowest bids, lowest to highest asks
    pub bids: Vec<(Decimal, Decimal)>,
    pub asks: Vec<(Decimal, Decimal)>,
    pub checksum: Checksum,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    pub time: DateTime<Utc>, // API returns 1621740952.5079553
}

type Checksum = u32;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Orderbook {
    initialized: bool,
    pub symbol: Symbol,
    pub bids: BTreeMap<Decimal, Decimal>,
    pub asks: BTreeMap<Decimal, Decimal>,
}

fn format_value(value: &Decimal) -> String {
    if value.fract().is_zero() {
        format!("{value:.1}")
    } else if *value < dec!(0.0001) {
        let mut formatted = format!("{value:e}");
        let minus_idx = formatted
            .find('-')
            .expect("Passed abs(value) higher than 1");
        formatted.insert(minus_idx + 1, '0');
        formatted
    } else {
        value.to_string()
    }
}

impl Orderbook {
    pub fn new(symbol: Symbol) -> Orderbook {
        Orderbook {
            symbol,
            initialized: false,
            bids: Default::default(),
            asks: Default::default(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn apply(&mut self, data: &OrderbookData) -> Result<(), Error> {
        self.bids.extend(data.bids.iter().cloned());
        self.asks.extend(data.asks.iter().cloned());

        self.bids.retain(|_k, v| v.is_zero().not());
        self.asks.retain(|_k, v| v.is_zero().not());

        if self.verify_checksum(&data.checksum) {
            Ok(())
        } else {
            Err(Error::IncorrectChecksum)
        }
    }

    pub fn update(&mut self, data: &OrderbookData) -> Result<(), Error> {
        if self.is_initialized() {
            self.apply(data)
        } else if data.action == OrderbookAction::Partial {
            self.initialized = true;
            self.apply(data)
        } else {
            Err(Error::MissingPartial)
        }
    }

    pub fn verify_checksum(&self, checksum: &Checksum) -> bool {
        let input = (0..100)
            .into_iter()
            .zip(self.bids.iter().rev().zip(self.asks.iter()))
            .map(|(_, ((b_p, b_s), (a_p, a_s)))| {
                vec![
                    format_value(b_p),
                    format_value(b_s),
                    format_value(a_p),
                    format_value(a_s),
                ]
                .join(":")
            })
            .collect::<Vec<String>>()
            .join(":");

        let input = input.as_bytes();

        let mut hasher = Hasher::new();
        hasher.update(input);
        let output = hasher.finalize();

        // println!("Output: {}, Checksum: {}", output, checksum);
        output == *checksum
    }

    /// Returns the price of the best bid
    pub fn bid_price(&self) -> Option<&Decimal> {
        self.bids.keys().next_back()
    }

    /// Returns the price of the best ask
    pub fn ask_price(&self) -> Option<&Decimal> {
        self.asks.keys().next()
    }

    /// Returns the midpoint between the best bid price and best ask price.
    /// Output is not rounded to the smallest price increment.
    pub fn mid_price(&self) -> Option<Decimal> {
        Some((self.bid_price()? + self.ask_price()?) / dec!(2))
    }

    /// Returns the price and quantity of the best bid
    /// (bid_price, bid_quantity)
    pub fn best_bid(&self) -> Option<(&Decimal, &Decimal)> {
        self.bids.iter().next_back()
    }

    /// Returns the price and quantity of the best ask
    /// (ask_price, ask_quantity)
    pub fn best_ask(&self) -> Option<(&Decimal, &Decimal)> {
        self.asks.iter().next()
    }

    /// Returns the price and quantity of the best bid and best ask
    /// ((bid_price, bid_quantity), (ask_price, ask_quantity))
    #[allow(clippy::type_complexity)]
    pub fn best_bid_and_ask(&self) -> Option<((&Decimal, &Decimal), (&Decimal, &Decimal))> {
        Some((self.best_bid()?, self.best_ask()?))
    }

    /// Returns the expected execution price of a market order given the current
    /// orders in the order book. Returns None if the order size exceeds the
    /// liquidity available on that side of the order book.
    pub fn quote(&self, side: Side, quantity: Decimal) -> Option<Decimal> {
        // Step 1: Match with orders in the book
        let mut bids_iter = self.bids.iter().rev();
        let mut asks_iter = self.asks.iter();

        let mut fills: Vec<(Decimal, Decimal)> = Vec::new(); // (price, quantity)
        let mut remaining = quantity;

        while remaining.is_zero().not() && remaining.is_sign_positive() {
            let (price, quantity) = match side {
                Side::Buy => asks_iter.next()?,
                Side::Sell => bids_iter.next()?,
            };

            if *quantity <= remaining {
                remaining -= quantity;
                fills.push((*price, *quantity));
            } else {
                fills.push((*price, remaining));
                remaining = dec!(0);
            }
        }

        // Step 2: Compute the weighted average
        let dot_product = fills
            .iter()
            .fold(dec!(0), |acc, (price, quantity)| acc + (price * quantity));

        Some(dot_product / quantity)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub id: Id,
    pub market: Option<Symbol>,
    pub future: Option<Symbol>,
    pub base_currency: Option<Coin>,
    pub quote_currency: Option<Coin>,
    pub r#type: String, // e.g. "order"
    pub side: Side,
    pub price: Decimal,
    pub size: Decimal,
    pub order_id: Option<Id>,
    pub trade_id: Option<Id>,
    pub time: DateTime<Utc>,
    pub fee: Decimal,
    pub fee_rate: Decimal,
    pub fee_currency: Coin,
    pub liquidity: Liquidity,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Liquidity {
    Maker,
    Taker,
}

#[cfg(test)]
mod tests {
    use super::*;

    // check examples from https://docs.ftx.com/#orderbooks
    #[test]
    fn test_format_value() {
        assert_eq!(&format_value(&dec!(0.000075)), "7.5e-05");
        assert_eq!(&format_value(&dec!(0.1)), "0.1");
    }
}
