use super::common::{Coin, DepositStatus, Id, WithdrawStatus};
use super::Request;
use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
    pub size: Option<Decimal>,
    pub time: String,
    pub status: DepositStatus,
    pub confirmations: Option<usize>,
    pub confirmed_time: Option<String>,
    pub fee: Option<Decimal>, // fee, not included in size
    pub txid: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletDeposits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
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

impl Request for GetWalletDeposits {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/wallet/deposits";
    const AUTH: bool = true;

    type Response = Vec<WalletDeposit>;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletBalances {}

impl Request for GetWalletBalances {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/wallet/balances";
    const AUTH: bool = true;

    type Response = Vec<WalletBalance>;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletDepositAddress {
    #[serde(skip_serializing)]
    pub coin: String,
    pub method: Option<String>,
}

impl GetWalletDepositAddress {
    pub fn new(coin: &str) -> Self {
        Self {
            coin: coin.into(),
            ..Default::default()
        }
    }

    pub fn with_method(coin: &str, method: &str) -> Self {
        Self {
            coin: coin.into(),
            method: Some(method.into()),
        }
    }
}

impl Request for GetWalletDepositAddress {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/wallet/deposit_address";
    const AUTH: bool = true;

    type Response = WalletDepositAddress;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/wallet/deposit_address/{}", self.coin))
    }
}

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

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCoins {}

impl Request for GetCoins {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/wallet/coins";
    const AUTH: bool = false;

    type Response = Vec<CoinInfo>;
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletWithdrawal {
    // Exclude `id` for now.  For FTX Card withdrawals `id` is unfortunately returned as an
    // alphanumeric `String` (eg. `"swipe_170108"`) instead of a number.
    /*pub id: Id,*/
    pub coin: String,
    pub size: Decimal,
    pub time: String,
    pub address: Option<String>, // `None` for transfers between sub-accounts
    pub status: WithdrawStatus,
    pub fee: Option<Decimal>, // fee, not included in size
    pub txid: Option<String>,
    pub tag: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletWithdrawals {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
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

impl Request for GetWalletWithdrawals {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/wallet/withdrawals";
    const AUTH: bool = true;

    type Response = Vec<WalletWithdrawal>;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestWithdrawal {
    pub coin: String,
    pub size: Decimal,
    pub address: String,
    pub tag: Option<String>,
    pub method: Option<String>,
    pub password: Option<String>,
    pub code: Option<String>,
}

impl Request for RequestWithdrawal {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/wallet/withdrawals";
    const AUTH: bool = true;

    type Response = WalletWithdrawal;
}
