mod orderbook;

use crate::rest::Rest;
use std::sync::Arc;
use tokio::sync::Mutex;
use orderbook::Orderbook;
use rust_decimal::prelude::*;

/// Represents some market.
#[derive(Clone)]
pub struct Market(Arc<Mutex<InternalMarket>>);

impl Market {
    // Returns the orderbook of this market.
    pub async fn orderbook(&self) -> Orderbook {
        self.0.lock().await.orderbook.clone()
    }

    pub async fn order(&self) {
        
    }
}

struct InternalMarket {
    rest: Rest,
    orderbook: Orderbook,
    price_increment: Decimal,
    size_increment: Decimal,
}