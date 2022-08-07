use super::common::Position;
use super::Request;
use http::Method;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

/// Returned by GET /account.
/// See https://docs.ftx.com/#get-account-information.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub backstop_provider: bool,
    pub charge_interest_on_negative_usd: bool,
    pub collateral: Decimal,
    pub free_collateral: Decimal,
    pub initial_margin_requirement: Decimal,
    pub liquidating: bool,
    pub maintenance_margin_requirement: Decimal,
    pub maker_fee: Decimal,
    pub margin_fraction: Option<Decimal>,
    pub open_margin_fraction: Option<Decimal>,
    pub position_limit: Option<Decimal>,
    pub position_limit_used: Option<Decimal>,
    pub taker_fee: Decimal,
    pub total_account_value: Decimal,
    pub total_position_size: Decimal,
    pub use_ftt_collateral: bool,
    pub username: String,
    pub leverage: Decimal,
    pub positions: Vec<Position>,
    pub spot_lending_enabled: bool,
    pub spot_margin_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetAccount {}

impl Request for GetAccount {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/account";
    const AUTH: bool = true;

    type Response = Account;
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChangeAccountLeverage {
    pub leverage: u32,
}

impl ChangeAccountLeverage {
    pub fn new(leverage: u32) -> Self {
        Self { leverage }
    }
}

impl Request for ChangeAccountLeverage {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/account/leverage";
    const AUTH: bool = true;

    type Response = ();
}
