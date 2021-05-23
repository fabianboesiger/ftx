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
    let mut ws = init_ws().await;

    ws.subscribe(vec![Channel::Orderbook("BTC-PERP".to_owned())])
        .await
        .expect("Subscription failed.");

    match ws.next().await.unwrap() {
        Some(Data::OrderBook(orderbook)) => {}
        _ => panic!("Order book data expected."),
    }
}
