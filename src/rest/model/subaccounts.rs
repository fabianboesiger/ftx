use super::{
    common::{Coin, Id},
    Request,
};
use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subaccount {
    pub nickname: String,
    pub deletable: bool,
    pub editable: bool,
    pub competition: bool,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetSubAccountsRequest;

pub type GetSubAccountsResponse = Vec<Subaccount>;

impl Request for GetSubAccountsRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/subaccounts";
    const AUTH: bool = true;

    type Response = GetSubAccountsResponse;
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateSubAccountRequest {
    pub nickname: String,
}

impl CreateSubAccountRequest {
    pub fn new(nickname: &str) -> Self {
        Self {
            nickname: nickname.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountResponse {
    pub nickname: String,
    pub deletable: bool,
    pub editable: bool,
}

impl Request for CreateSubAccountRequest {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/subaccounts";
    const AUTH: bool = true;

    type Response = CreateSubAccountResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSubaccountNameRequest {
    pub nickname: String,
    pub new_nickname: String,
}

impl ChangeSubaccountNameRequest {
    pub fn new(nickname: &str, new_nickname: &str) -> Self {
        Self {
            nickname: nickname.into(),
            new_nickname: new_nickname.into(),
        }
    }
}

pub type ChangeSubaccountNameResponse = ();

impl Request for ChangeSubaccountNameRequest {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/subaccounts/update_name";
    const AUTH: bool = true;

    type Response = ChangeSubaccountNameResponse;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubaccountRequest {
    pub nickname: String,
}

impl DeleteSubaccountRequest {
    pub fn new(nickname: &str) -> Self {
        Self {
            nickname: nickname.into(),
        }
    }
}

pub type DeleteSubaccountResponse = ();

impl Request for DeleteSubaccountRequest {
    const METHOD: Method = Method::DELETE;
    const PATH: &'static str = "/subaccounts";
    const AUTH: bool = true;

    type Response = DeleteSubaccountResponse;
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub coin: Coin,
    pub free: Decimal,
    pub total: Decimal,
    pub spot_borrow: Decimal,
    pub available_without_borrow: Decimal,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSubaccountBalancesRequest {
    #[serde(skip_serializing)]
    pub nickname: String,
}

impl GetSubaccountBalancesRequest {
    pub fn new(nickname: &str) -> Self {
        Self {
            nickname: nickname.into(),
        }
    }
}

pub type GetSubaccountBalancesResponse = Vec<Balance>;

impl Request for GetSubaccountBalancesRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/subaccounts/{}/balances";
    const AUTH: bool = true;

    type Response = GetSubaccountBalancesResponse;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/subaccounts/{}/balances", self.nickname))
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub id: Id,
    pub coin: Coin,
    pub size: Decimal,
    pub time: DateTime<Utc>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransferBetweenSubaccountsRequest {
    pub coin: String,
    pub size: Decimal,
    pub source: String,
    pub destination: String,
}

impl TransferBetweenSubaccountsRequest {
    pub fn new<S>(coin: &str, size: S, source: &str, destination: &str) -> Self
    where
        Decimal: TryFrom<S>,
        <Decimal as TryFrom<S>>::Error: Debug,
    {
        Self {
            coin: coin.into(),
            size: Decimal::try_from(size).unwrap(),
            source: source.into(),
            destination: destination.into(),
        }
    }
}

pub type TransferBetweenSubaccountsResponse = Transfer;

impl Request for TransferBetweenSubaccountsRequest {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/subaccounts/transfer";
    const AUTH: bool = true;

    type Response = TransferBetweenSubaccountsResponse;
}
