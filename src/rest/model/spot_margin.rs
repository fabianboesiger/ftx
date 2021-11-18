use {
    super::Request,
    http::Method,
    rust_decimal::Decimal,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetLendingInfo {}

#[derive(Clone, Debug, Deserialize)]
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
