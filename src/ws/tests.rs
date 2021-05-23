use super::*;
use dotenv::dotenv;
use std::env::var;

async fn init_ws() -> Ws {
    dotenv().ok();
    Ws::connect(
        var("API_KEY").expect("API Key is not defined."),
        var("API_SECRET").expect("API Secret is not defined."),
        var("SUBACCOUNT").ok(),
    )
    .await
    .expect("Connection failed.")
}

#[tokio::test]
async fn trades() {
    let mut ws = init_ws().await;

    ws.subscribe(vec![Channel::Trades("BTC-PERP".to_owned())])
        .await
        .expect("Subscription failed.");

    match ws.next().await.unwrap() {
        Some(Data::Trade(..)) => {}
        _ => panic!("Trade data expected."),
    }
}

#[tokio::test]
async fn order_book() {
    let mut orderbook = OrderBook::new();

    let mut ws = init_ws().await;

    ws.subscribe(vec![Channel::Orderbook("BTC-PERP".to_owned())])
        .await
        .expect("Subscription failed.");

    // The initial snapshot of the data
    match ws.next().await.unwrap() {
        Some(Data::OrderBookData(orderbook_data))
        if orderbook_data.action == OrderBookAction::Partial => {

            for bid in &orderbook_data.bids {
                orderbook.bids.insert(bid.0, bid.1);
            }
            for ask in &orderbook_data.asks {
                orderbook.asks.insert(ask.0, ask.1);
            }
            // println!("{:#?}", orderbook);
        }
        _ => panic!("Order book snapshot data expected."),
    }
}
