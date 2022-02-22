use {
    super::Request,
    chrono::{DateTime, Utc},
    http::Method,
    rust_decimal::Decimal,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetLendingRates {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LendingRate {
    pub coin: String,
    pub estimate: Decimal, // estimated hourly lending rate for the next spot margin cycle
    pub previous: Decimal, // hourly lending rate in the previous spot margin cycle
}

impl Request for GetLendingRates {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/spot_margin/lending_rates";
    const AUTH: bool = true;

    type Response = Vec<LendingRate>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyLendingHistory {
    pub coin: String,
    pub proceeds: Decimal,
    pub rate: Decimal,
    pub size: Decimal,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMyLendingHistory {
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

impl Request for GetMyLendingHistory {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/spot_margin/lending_history";
    const AUTH: bool = true;

    type Response = Vec<MyLendingHistory>;
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetLendingInfo {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LendingInfo {
    pub coin: String,
    pub lendable: Decimal,
    pub locked: Decimal,
    pub min_rate: Option<Decimal>,
    pub offered: Decimal,
}

impl Request for GetLendingInfo {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/spot_margin/lending_info";
    const AUTH: bool = true;

    type Response = Vec<LendingInfo>;
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct SubmitLendingOffer {
    pub coin: String,
    pub size: Decimal,
    pub rate: Decimal,
}

impl Request for SubmitLendingOffer {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/spot_margin/offers";
    const AUTH: bool = true;

    type Response = ();
}
