use super::common::Side;
use super::Request;
use http::Method;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

/// Returned by GET /positions.
/// See https://docs.ftx.com/#get-positions.

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub cost: Decimal,
    pub entry_price: Option<Decimal>,
    pub estimated_liquidation_price: Option<Decimal>,
    pub future: String,
    pub initial_margin_requirement: Decimal,
    pub long_order_size: Decimal,
    pub maintenance_margin_requirement: Decimal,
    pub net_size: Decimal,
    pub open_size: Decimal,
    pub realized_pnl: Decimal,
    pub short_order_size: Decimal,
    pub side: Side,
    pub size: Decimal,
    pub unrealized_pnl: Decimal,
    pub collateral_used: Decimal,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionsRequest;

pub type GetPositionsResponse = Vec<Position>;

impl Request for GetPositionsRequest {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/positions";
    const HAS_PAYLOAD: bool = false;
    const AUTH: bool = true;

    type Response = GetPositionsResponse;
}
