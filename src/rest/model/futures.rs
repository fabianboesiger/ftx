use super::common::{FutureType, Symbol};
use super::Request;
use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Future {
    pub ask: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub change1h: Option<Decimal>,
    pub change24h: Option<Decimal>,
    pub change_bod: Option<Decimal>,
    pub volume_usd24h: Option<Decimal>,
    pub volume: Option<Decimal>,
    pub description: String,
    pub enabled: bool,
    pub expired: bool,
    pub expiry: Option<DateTime<Utc>>,
    pub index: Option<Decimal>,
    pub imf_factor: Decimal,
    pub last: Option<Decimal>,
    pub lower_bound: Decimal,
    pub mark: Option<Decimal>,
    pub name: Symbol,
    pub perpetual: bool,
    pub position_limit_weight: Decimal,
    pub post_only: bool,
    pub price_increment: Decimal,
    pub size_increment: Decimal,
    pub underlying: Symbol,
    pub upper_bound: Decimal,
    #[serde(rename = "type")]
    pub market_type: FutureType,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFutures;

impl Request for GetFutures {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/futures";
    const AUTH: bool = false;

    type Response = Vec<Future>;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFuture {
    #[serde(skip_serializing)]
    pub future_name: String,
}

impl GetFuture {
    pub fn new(future_name: &str) -> Self {
        Self {
            future_name: future_name.into(),
        }
    }
}

impl Request for GetFuture {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/futures/{}";
    const AUTH: bool = false;

    type Response = Future;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/futures/{}", self.future_name))
    }
}
