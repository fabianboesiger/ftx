pub use crate::rest::{Coin, Id, MarketType, OrderInfo, Side, Symbol, Trade};
use chrono::{DateTime, Utc};
use crc32fast::Hasher;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSecondsWithFrac};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
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
                self.bids.clear();
                self.asks.clear();
                for bid in &data.bids {
                    self.bids.insert(bid.0, bid.1);
                }
                for ask in &data.asks {
                    self.asks.insert(ask.0, ask.1);
                }
            }
            OrderbookAction::Update => {
                for bid in &data.bids {
                    if bid.1 == dec!(0) {
                        self.bids.remove(&bid.0);
                    } else {
                        self.bids.insert(bid.0, bid.1);
                    }
                }
                for ask in &data.asks {
                    if ask.1 == dec!(0) {
                        self.asks.remove(&ask.0);
                    } else {
                        self.asks.insert(ask.0, ask.1);
                    }
                }
            }
        }
    }

    /// Internal helper function that serializes Decimal to String,
    /// padding a 0 if the Decimal is a whole number
    fn _pad_0(&self, value: Decimal) -> String {
        if value.fract() == dec!(0) {
            format!("{:.1}", value)
        } else {
            value.to_string()
        }
    }

    pub fn verify_checksum(&self, checksum: Checksum) -> bool {
        let mut input: Vec<String> = Vec::new();

        let mut bids_iter = self.bids.iter().rev();
        let mut asks_iter = self.asks.iter();

        for _i in 0..100 {
            let bid = bids_iter.next();
            let ask = asks_iter.next();

            if let Some(bid) = bid {
                let bid_price = self._pad_0(*bid.0);
                let bid_quantity = self._pad_0(*bid.1);
                input.push(format!("{}:{}", bid_price, bid_quantity));
            }
            if let Some(ask) = ask {
                let ask_price = self._pad_0(*ask.0);
                let ask_quantity = self._pad_0(*ask.1);
                input.push(format!("{}:{}", ask_price, ask_quantity));
            }
        }

        let input: String = input.join(":");
        // println!("{}", input);
        let input = input.as_bytes();

        let mut hasher = Hasher::new();
        hasher.update(input);
        let output = hasher.finalize();

        // println!("Output: {}, Checksum: {}", output, checksum);
        output == checksum
    }

    /// Returns the price of the best bid
    pub fn bid_price(&self) -> Option<Decimal> {
        self.bids.keys().rev().next().cloned()
    }

    /// Returns the price of the best ask
    pub fn ask_price(&self) -> Option<Decimal> {
        self.asks.keys().next().cloned()
    }

    /// Returns the midpoint between the best bid price and best ask price.
    /// Output is not rounded to the smallest price increment.
    pub fn mid_price(&self) -> Option<Decimal> {
        Some((self.bid_price()? + self.ask_price()?) / dec!(2))
    }

    /// Returns the price and quantity of the best bid
    /// (bid_price, bid_quantity)
    pub fn best_bid(&self) -> Option<(Decimal, Decimal)> {
        let (price, quantity) = self.bids.iter().rev().next()?;

        Some((*price, *quantity))
    }

    /// Returns the price and quantity of the best ask
    /// (ask_price, ask_quantity)
    pub fn best_ask(&self) -> Option<(Decimal, Decimal)> {
        let (price, quantity) = self.asks.iter().next()?;

        Some((*price, *quantity))
    }

    /// Returns the price and quantity of the best bid and best ask
    /// ((bid_price, bid_quantity), (ask_price, ask_quantity))
    pub fn best_bid_and_ask(&self) -> Option<((Decimal, Decimal), (Decimal, Decimal))> {
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

        while remaining > dec!(0) {
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
        let mut dot_product = dec!(0);
        for (fill_price, fill_quantity) in fills.iter() {
            dot_product += fill_price * fill_quantity;
        }

        Some(dot_product / quantity)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub id: Id,
    pub market: Symbol,
    pub future: Option<Symbol>,
    pub base_currency: Option<Coin>,
    pub quote_currency: Option<Coin>,
    pub r#type: String, // e.g. "order"
    pub side: Side,
    pub price: Decimal,
    pub size: Decimal,
    pub order_id: Id,
    pub trade_id: Id,
    pub time: DateTime<Utc>,
    pub fee: Decimal,
    pub fee_rate: Decimal,
    pub fee_currency: Coin,
    pub liquidity: Liquidity,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Liquidity {
    Maker,
    Taker,
}
