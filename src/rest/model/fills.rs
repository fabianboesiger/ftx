use super::Request;
use crate::ws::Fill;
use http::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFills {
    #[serde(skip_serializing)]
    pub market_name: String,
}

impl GetFills {
    pub fn new(market_name: &str) -> Self {
        Self {
            market_name: market_name.into(),
        }
    }
}

impl Request for GetFills {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/fills?market={}";
    const AUTH: bool = true;

    type Response = Vec<Fill>;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/fills?market={}", self.market_name))
    }
}
