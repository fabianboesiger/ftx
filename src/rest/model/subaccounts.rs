use super::Request;
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetSubAccountsRequest;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subaccount {
    pub nickname: String,
    pub deletable: bool,
    pub editable: bool,
    pub competition: bool,
}

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
