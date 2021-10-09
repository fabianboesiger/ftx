use super::Request;
use http::Method;
use serde::{Deserialize, Serialize};

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
    const HAS_PAYLOAD: bool = false;
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
    const HAS_PAYLOAD: bool = true;
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
    const HAS_PAYLOAD: bool = true;
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
    const HAS_PAYLOAD: bool = true;
    const AUTH: bool = true;

    type Response = DeleteSubaccountResponse;
}
