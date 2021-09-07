use serde::{Deserialize, Serialize};

pub type Id = u64;
pub type Coin = String;
pub type Symbol = String;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    TrailingStop,
    TakeProfit,
}

impl Default for OrderType {
    fn default() -> OrderType {
        OrderType::Market
    }
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
/// Represents the status of the order.
/// However, the REST and websockets APIs assign these values differently.
///
/// When submitting orders over REST, the API will immediately return whether
/// the order is accepted into FTX's queue for processing, but not the results
/// of the processing. If the order is accepted into the queue, the API will
/// return an `OrderInfo` with `OrderStatus::New`, otherwise it will return an error.
///
/// If the order is rejected during processing (e.g. when submitting a post-only
/// limit order with a price that would be executed as a taker order), the user
/// will not know this unless they do one of the following:
/// - Call the `get_order` REST API to see if the order status has been updated
/// - Listen to orders over websockets to be notified of the update order status
///   as soon as it is available.
/// To get near-immediate feedback on the status of possibly-rejected orders,
/// we recommend subscribing to the `Orders` channel over websockets.
///
/// When listening to orders over websockets, the websockets API will report
/// only the status of the order when it has been processed:
/// - If an order is rejected upon processing, the websockets API will emit an
///   `OrderInfo` with `OrderStatus::Closed`. Unlike the REST API, it will not
///   return an `OrderInfo` with `OrderStatus::New`.
/// - If a limit order is accepted and not immediately filled upon processing,
///   the websockets API will emit an `OrderInfo` with `OrderStatus::New`,
///   confirming the order as active.
/// - If a limit or market order is accepted and filled immediately upon
///   processing, the websockets API emits an `OrderInfo` with
///   `OrderStatus::Closed`.
pub enum OrderStatus {
    /// Rest: accepted but not processed yet; Ws: processed and confirmed active
    New,
    /// Applicable to Rest only
    Open,
    /// Rest: filled or cancelled; Ws: filled, rejected, or cancelled
    Closed,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    Buy,
    Sell,
}

impl Default for Side {
    fn default() -> Side {
        Side::Buy
    }
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FutureType {
    Future,
    Perpetual,
    Prediction,
    Move,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DepositStatus {
    Confirmed,
    Unconfirmed,
    Cancelled,
    Complete,
}
