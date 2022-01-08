use super::{common::Id, Request};
use crate::ws::Fill;
use chrono::{DateTime, Utc};
use http::Method;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFills {
    pub market_name: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<Id>,
}

impl GetFills {
    pub fn new(market_name: &str) -> Self {
        Self {
            market_name: market_name.into(),
            ..Self::default()
        }
    }
}

impl Request for GetFills {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/fills";
    const AUTH: bool = true;

    type Response = Vec<Fill>;
}
