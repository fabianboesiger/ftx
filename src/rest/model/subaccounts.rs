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
pub struct CreateSubaccount {
    pub nickname: String,
}

impl CreateSubaccount {
    pub fn new(nickname: &str) -> Self {
        Self {
            nickname: nickname.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    pub nickname: String,
    pub deletable: bool,
    pub editable: bool,
}

impl Request for CreateSubaccount {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/subaccounts";
    const AUTH: bool = true;

    type Response = Create;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSubaccountName {
    pub nickname: String,
    pub new_nickname: String,
}

impl ChangeSubaccountName {
    pub fn new(nickname: &str, new_nickname: &str) -> Self {
        Self {
            nickname: nickname.into(),
            new_nickname: new_nickname.into(),
        }
    }
}

impl Request for ChangeSubaccountName {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/subaccounts/update_name";
    const AUTH: bool = true;

    type Response = ();
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubaccount {
    pub nickname: String,
}

impl DeleteSubaccount {
    pub fn new(nickname: &str) -> Self {
        Self {
            nickname: nickname.into(),
        }
    }
}

impl Request for DeleteSubaccount {
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
pub struct GetSubaccountBalances {
    #[serde(skip_serializing)]
    pub nickname: String,
}

impl GetSubaccountBalances {
    pub fn new(nickname: &str) -> Self {
        Self {
            nickname: nickname.into(),
        }
    }
}

impl Request for GetSubaccountBalances {
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
pub struct TransferBetweenSubaccounts {
    pub coin: String,
    pub size: Decimal,
    pub source: String,
    pub destination: String,
}

impl TransferBetweenSubaccounts {
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

impl Request for TransferBetweenSubaccounts {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/subaccounts/transfer";
    const AUTH: bool = true;

    type Response = Transfer;
}
