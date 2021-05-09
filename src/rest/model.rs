use chrono::{DateTime, Utc};
use rust_decimal::prelude::*;
use serde::Deserialize;

pub type Id = u64;
pub type Coin = String;
pub type Symbol = String;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Response<T> {
    Result { success: bool, result: T },
    Error { success: bool, error: String },
}

// REST API -> Subaccounts

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subaccount {
    pub nickname: String,
    pub deletable: bool,
    pub editable: bool,
    pub competition: bool,
}

pub type Subaccounts = Vec<Subaccount>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    pub nickname: String,
    pub deletable: bool,
    pub editable: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeName;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delete;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub coin: Coin,
    pub free: Decimal,
    pub total: Decimal,
    pub spot_borrow: Decimal,
    pub available_without_borrow: Decimal,
}

pub type Balances = Vec<Balance>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub id: Id,
    pub coin: Coin,
    pub size: Decimal,
    pub time: DateTime<Utc>,
    pub notes: String,
}

// REST API -> Markets

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarketType {
    Future,
    Spot,
}

#[derive(Debug, Deserialize)]
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
    pub last: Decimal,
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

pub type Markets = Vec<Market>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    pub asks: Vec<(Decimal, Decimal)>,
    pub bids: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: Id,
    pub liquidation: bool,
    pub price: Decimal,
    pub side: Side,
    pub size: Decimal,
    pub time: DateTime<Utc>,
}

pub type Trades = Vec<Trade>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub open: Decimal,
    pub volume: Decimal,
    pub start_time: DateTime<Utc>,
}

pub type Prices = Vec<Price>;

// REST API -> Futures

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FutureType {
    Future,
    Perpetual,
    Move,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Future {
    pub ask: Decimal,
    pub bid: Decimal,
    pub change1h: Decimal,
    pub change24h: Decimal,
    //pub change_bod: Decimal,
    //pub volume_usd24h: Decimal,
    //pub volume: Decimal,
    pub description: String,
    pub enabled: bool,
    pub expired: bool,
    pub expiry: DateTime<Utc>,
    pub index: Decimal,
    //pub imf_factor: Decimal,
    pub last: Decimal,
    pub lower_bound: Decimal,
    pub mark: Decimal,
    pub name: Symbol,
    pub perpetual: bool,
    //pub position_limit_weight: Decimal,
    pub post_only: bool,
    pub price_increment: Decimal,
    pub size_increment: Decimal,
    pub underlying: Symbol,
    pub upper_bound: Decimal,
    #[serde(rename = "type")]
    pub market_type: FutureType,
}

pub type Futures = Vec<Future>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureStats {
    pub volume: Decimal,
    pub next_funding_rate: Decimal,
    pub next_funding_time: DateTime<Utc>,
    pub expiration_price: Decimal,
    pub predicted_expiration_price: Decimal,
    pub strike_price: Decimal,
    pub open_interest: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub future: Symbol,
    pub rate: Decimal,
    pub time: DateTime<Utc>,
}

pub type FundingRates = Vec<FundingRate>;

// REST API -> Account

/// Returned by GET /account.
/// See https://docs.ftx.com/#get-account-information.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub backstop_provider: bool,
    pub charge_interest_on_negative_usd: bool,
    pub collateral: Decimal,
    pub free_collateral: Decimal,
    pub initial_margin_requirement: Decimal,
    pub liquidating: bool,
    pub maintenance_margin_requirement: Decimal,
    pub maker_fee: Decimal,
    pub margin_fraction: Decimal,
    pub open_margin_fraction: Decimal,
    pub position_limit: Option<Decimal>,
    pub position_limit_used: Option<Decimal>,
    pub taker_fee: Decimal,
    pub total_account_value: Decimal,
    pub total_position_size: Decimal,
    pub use_ftt_collateral: bool,
    pub username: String,
    pub leverage: Decimal,
    pub positions: Vec<Position>,
    pub spot_lending_enabled: bool,
    pub spot_margin_enabled: bool,
}

/// Returned by GET /positions.
/// See https://docs.ftx.com/#get-positions.
pub type Positions = Vec<Position>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub cost: Decimal,
    pub entry_price: Option<Decimal>,
    pub estimated_liquidation_price: Option<Decimal>,
    pub future: String,
    pub initial_margin_requirement: Decimal,
    pub long_order_size: Decimal,
    pub maintenance_margin_requirement: Decimal,
    pub net_size: Decimal,
    pub open_size: Decimal,
    pub realized_pnl: Decimal,
    pub short_order_size: Decimal,
    pub side: Side,
    pub size: Decimal,
    pub unrealized_pnl: Decimal,
    pub collateral_used: Decimal,
}

// REST API -> Wallet
// TODO

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletDepositAddress {
    pub address: String,
    pub tag: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    pub coin: String,
    pub free: f64,
    pub total: f64,
    pub spot_borrow: f64,
    pub available_without_borrow: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletDeposit {
    pub coin: String,
    pub confirmations: usize,
    pub confirmed_time: String,
    pub fee: f64, // fee, not included in size
    pub id: usize,
    pub size: f64,
    pub status: String, // "confirmed", "unconfirmed", or "cancelled"
    pub time: String,
    pub txid: Option<String>,
    pub notes: Option<String>,
}

// REST API -> Orders
// TODO
