use super::{common::Id, Request};
use crate::ws::Fill;
use chrono::{DateTime, Utc};
use http::Method;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]

pub struct GetFills<'a> {
    #[serde(rename = "marketName")]
    pub market_name: &'a str,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "orderId")]
    pub order_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl<'a> GetFills<'a> {
    pub fn new(market_name: &'a str) -> Self {
        Self {
            market_name,
            ..Self::default()
        }
    }
}

impl Request for GetFills<'_> {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/fills";
    const AUTH: bool = true;

    type Response = Vec<Fill>;
}
