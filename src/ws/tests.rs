use super::*;
use dotenv::dotenv;
use std::env::var;

async fn init_ws() -> Ws {
    dotenv().ok();
    Ws::connect(
        var("API_KEY").expect("API Key is not defined."),
        var("API_SECRET").expect("API Secret is not defined."),
    )
    .await
    .expect("Connection failed.")
}

#[tokio::test]
async fn trades() {
    let mut ws = init_ws().await;

    ws.subscribe(Channel::Trades, "BTC/USD")
        .await
        .expect("Subscription failed.");

    ws.next().await.unwrap();
}
