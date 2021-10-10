mod account;
mod common;
mod futures;
mod markets;
mod orders;
mod positions;
mod subaccounts;
mod wallet;

pub use self::account::*;
pub use self::common::*;
pub use self::futures::*;
pub use self::markets::*;
pub use self::orders::*;
pub use self::positions::*;
pub use self::subaccounts::*;
pub use self::wallet::*;

use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Request: Serialize {
    const METHOD: Method;
    const PATH: &'static str;
    const HAS_PAYLOAD: bool = false;
    const AUTH: bool = false;

    type Response: DeserializeOwned;

    fn no_payload(&self) -> bool {
        !Self::HAS_PAYLOAD
    }

    fn path(&self) -> String {
        Self::PATH.into()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub result: T,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

// REST API -> Markets

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureStats {
    pub volume: Decimal,
    pub next_funding_rate: Decimal,
    pub next_funding_time: DateTime<Utc>,
    pub expiration_price: Decimal,
    pub predicted_expiration_price: Decimal,
    pub strike_price: Decimal,
    pub open_interest: Decimal,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub future: Symbol,
    pub rate: Decimal,
    pub time: DateTime<Utc>,
}

pub type FundingRates = Vec<FundingRate>;
