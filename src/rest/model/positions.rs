use super::common::Position;
use super::Request;
use http::Method;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionsRequest;

pub type GetPositionsResponse = Vec<Position>;

impl Request for GetPositionsRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/positions";
    const HAS_PAYLOAD: bool = false;
    const AUTH: bool = true;

    type Response = GetPositionsResponse;
}
