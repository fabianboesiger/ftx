use super::common::{DepositStatus, Id};
use super::Request;
use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletDepositsRequest {
    pub limit: Option<usize>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

pub type GetWalletDepositsResponse = Vec<WalletDeposit>;

impl Request for GetWalletDepositsRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/wallet/deposits";
    const HAS_PAYLOAD: bool = true;
    const AUTH: bool = true;

    type Response = GetWalletDepositsResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletBalancesRequest;

pub type GetWalletBalancesResponse = Vec<WalletBalance>;

impl Request for GetWalletBalancesRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/wallet/balances";
    const HAS_PAYLOAD: bool = false;
    const AUTH: bool = true;

    type Response = GetWalletBalancesResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletDepositAddressRequest {
    pub coin: String,
    pub method: Option<String>,
}

impl GetWalletDepositAddressRequest {
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

pub type GetWalletDepositAddressResponse = WalletDepositAddress;

impl Request for GetWalletDepositAddressRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/wallet/deposit_address";
    const HAS_PAYLOAD: bool = true;
    const AUTH: bool = true;

    type Response = GetWalletDepositAddressResponse;
}
