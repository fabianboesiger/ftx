mod account;
mod common;
mod futures;
mod markets;
mod orders;
mod positions;
mod subaccounts;
mod wallet;

pub use self::account::*;
pub use self::common::*;
pub use self::futures::*;
pub use self::markets::*;
pub use self::orders::*;
pub use self::positions::*;
pub use self::subaccounts::*;
pub use self::wallet::*;

use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Request: Serialize {
    const METHOD: Method;
    const PATH: &'static str;
    const HAS_PAYLOAD: bool = false;
    const AUTH: bool = false;

    type Response: DeserializeOwned;

    fn no_payload(&self) -> bool {
        !Self::HAS_PAYLOAD
    }

    fn path(&self) -> String {
        Self::PATH.into()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub result: T,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

// REST API -> Markets

#[derive(Copy, Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub future: Symbol,
    pub rate: Decimal,
    pub time: DateTime<Utc>,
}

pub type FundingRates = Vec<FundingRate>;

// REST API -> Account

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub id: Coin,     // "USDT"
    pub name: String, // "USD Tether"
    pub collateral: bool,
    pub usd_fungible: bool,
    pub is_etf: bool, // Not documented
    pub is_token: bool,
    pub hidden: bool, // Not documented
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub can_convert: bool,
    pub has_tag: bool,
    pub collateral_weight: Decimal,
    pub fiat: bool,
    pub methods: Vec<String>, // ["omni", "erc20", "trx", "sol", "heco"]
    pub erc20_contract: Option<String>, // "0xdAC17F958D2ee523a2206206994597C13D831ec7"
    pub bep2_asset: Option<String>, // "ETHBEAR-B2B"
    pub trc20_contract: Option<String>, // "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t"
    pub spl_mint: Option<String>, // "BQcdHdAQW1hczDbBi9hiegXAR7A98Q9jx3X3iBBBDiq4"
    pub credit_to: Option<String>, // "USDT"
    pub spot_margin: bool,    // Not documented
    pub tokenized_equity: Option<bool>, // Not documented
    pub index_price: Decimal, // Not documented; note that ~8% return 1e-8
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletDepositAddress {
    pub address: String,
    pub tag: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    pub coin: String,
    pub free: Decimal,
    pub total: Decimal,
    pub spot_borrow: Decimal,
    pub available_without_borrow: Decimal,
    /// As of 2021-05-12, usdValue is not documented on
    /// https://docs.ftx.com/#get-balances, but it is returned.
    pub usd_value: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletDeposit {
    pub id: Id,
    pub coin: String,
    pub size: Decimal,
    pub time: String,
    pub status: DepositStatus,
    pub confirmations: Option<usize>,
    pub confirmed_time: Option<String>,
    pub fee: Option<Decimal>, // fee, not included in size
    pub txid: Option<String>,
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WithdrawStatus {
    Requested,
    Processing,
    Complete,
    Cancelled,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletWithdrawal {
    pub id: Id,
    pub coin: String,
    pub size: Decimal,
    pub time: String,
    pub address: String,
    pub status: WithdrawStatus,
    pub fee: Option<Decimal>, // fee, not included in size
    pub txid: Option<String>,
    pub tag: Option<String>,
    pub notes: Option<String>,
}
