use super::common::Id;
use super::Request;
use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingPayment {
    pub id: Id,
    pub future: String,
    pub payment: Decimal,
    pub time: DateTime<Utc>,
}

type FundingPayments = Vec<FundingPayment>;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future: Option<String>,
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
}

impl Request for GetFundingPayments {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/funding_payments";
    const AUTH: bool = true;

    type Response = FundingPayments;
}
