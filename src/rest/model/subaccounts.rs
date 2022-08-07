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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subaccount {
    pub nickname: String,
    pub deletable: bool,
    pub editable: bool,
    pub competition: bool,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetSubaccounts {}

impl Request for GetSubaccounts {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/subaccounts";
    const AUTH: bool = true;

    type Response = Vec<Subaccount>;
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateSubaccount<'a> {
    pub nickname: &'a str,
}

impl<'a> CreateSubaccount<'a> {
    pub fn new(nickname: &'a str) -> Self {
        Self { nickname }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    pub nickname: String,
    pub deletable: bool,
    pub editable: bool,
}

impl Request for CreateSubaccount<'_> {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/subaccounts";
    const AUTH: bool = true;

    type Response = Create;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSubaccountName<'a> {
    pub nickname: &'a str,
    pub new_nickname: &'a str,
}

impl<'a> ChangeSubaccountName<'a> {
    pub fn new(nickname: &'a str, new_nickname: &'a str) -> Self {
        Self {
            nickname,
            new_nickname,
        }
    }
}

impl Request for ChangeSubaccountName<'_> {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/subaccounts/update_name";
    const AUTH: bool = true;

    type Response = ();
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubaccount<'a> {
    pub nickname: &'a str,
}

impl<'a> DeleteSubaccount<'a> {
    pub fn new(nickname: &'a str) -> Self {
        Self { nickname }
    }
}

impl Request for DeleteSubaccount<'_> {
    const METHOD: Method = Method::DELETE;
    const PATH: &'static str = "/subaccounts";
    const AUTH: bool = true;

    type Response = ();
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
pub struct GetSubaccountBalances<'a> {
    #[serde(skip_serializing)]
    pub nickname: &'a str,
}

impl<'a> GetSubaccountBalances<'a> {
    pub fn new(nickname: &'a str) -> Self {
        Self { nickname }
    }
}

impl Request for GetSubaccountBalances<'_> {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/subaccounts/{}/balances";
    const AUTH: bool = true;

    type Response = Vec<Balance>;

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
pub struct TransferBetweenSubaccounts<'a> {
    pub coin: &'a str,
    pub size: Decimal,
    pub source: &'a str,
    pub destination: &'a str,
}

// TODO: should this return a Result<> since it can fail?
impl<'a> TransferBetweenSubaccounts<'a> {
    pub fn new<S>(coin: &'a str, size: S, source: &'a str, destination: &'a str) -> Self
    where
        Decimal: TryFrom<S>,
        <Decimal as TryFrom<S>>::Error: Debug,
    {
        Self {
            coin,
            size: Decimal::try_from(size).unwrap(),
            source,
            destination,
        }
    }
}

impl Request for TransferBetweenSubaccounts<'_> {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/subaccounts/transfer";
    const AUTH: bool = true;

    type Response = Transfer;
}
