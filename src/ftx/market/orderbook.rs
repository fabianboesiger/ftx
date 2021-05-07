use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents the orderbook of some market.
#[derive(Clone)]
pub struct Orderbook(Arc<Mutex<InternalOrderbook>>);

struct InternalOrderbook {
}