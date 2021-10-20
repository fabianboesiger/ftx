use super::common::Position;
use super::Request;
use http::Method;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositions {}

impl Request for GetPositions {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/positions";
    const AUTH: bool = true;

    type Response = Vec<Position>;
}
