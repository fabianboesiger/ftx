use super::*;
use dotenv::dotenv;
use std::env::var;

async fn init_ws() -> Ws {
    dotenv().ok();
    Ws::connect(
        var("API_KEY").expect("API Key is not defined."),
        var("API_SECRET").expect("API Secret is not defined."),
    ).await.expect("Connection failed.")
}

#[tokio::test]
async fn trades() {
    let mut ws = init_ws().await;

    ws.subscribe(Channel::Trades, "BTC/USD")
        .await
        .expect("Subscription failed.");

    ws.next()
        .await
        .unwrap();
}

#[tokio::test]
async fn keep_alive() {
    let mut ws = init_ws().await;

    ws.subscribe(Channel::Trades, "BTC/USD")
        .await
        .expect("Subscription failed.");

    let sleep = tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60));
    tokio::pin!(sleep);

    while !sleep.is_elapsed() {
        tokio::select! {
            _ = &mut sleep, if !sleep.is_elapsed() => {
                println!("operation timed out");
            }
            _ = async {
                while let Ok(_) = ws.next().await {}
            } => {
                panic!("Websocket connection closed.");
            }
        }
    }
}

