//! This module keeps track of the entire state of markets,
//! orderbooks, wallets and so on by using both the REST and
//! the Websocket APIs. It provides an intuitive way to interact
//! with the FTX exchange.

mod error;
mod market;
mod wallet;

pub use error::*;
pub use market::*;
pub use wallet::*;

use crate::{
    rest::{Coin, Rest, Symbol},
    ws::Ws,
};
use error::Result;
use market::Market;
use rust_decimal::prelude::*;
use std::collections::HashMap;
use tokio::sync::{Mutex, MutexGuard};

pub struct Ftx {
    rest: Rest,
    ws: Ws,
    markets: Mutex<HashMap<Symbol, Market>>,
}

impl Ftx {
    pub async fn new(key: String, secret: String, subaccount: Option<String>) -> Result<Self> {
        let rest = Rest::new(key.clone(), secret.clone(), subaccount);
        let ws = Ws::connect(key, secret).await?;

        Ok(Self {
            rest,
            ws,
            markets: Mutex::new(HashMap::new()),
        })
    }

    /// Returns the market with the given symbol.
    pub async fn market<S: AsRef<Symbol>>(&self, symbol: S) -> Option<Market> {
        self.markets.lock().await.get(symbol.as_ref()).cloned()
    }
}
