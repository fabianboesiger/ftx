use super::*;
use dotenv::dotenv;
use std::env::var;
use rust_decimal::Decimal;

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

    let symbol: Symbol = String::from("BTC-PERP");
    ws.subscribe(vec![Channel::Orderbook(symbol.to_owned())])
        .await
        .expect("Subscription failed.");

    let mut orderbook = OrderBook::new(symbol);

    // The initial snapshot of the order book
    match ws.next().await.unwrap() {
        Some(Data::OrderBookData(data))

        if data.action == OrderBookAction::Partial => {
            for bid in &data.bids {
                orderbook.bids.insert(bid.0, bid.1);
            }
            for ask in &data.asks {
                orderbook.asks.insert(ask.0, ask.1);
            }
            // println!("{:#?}", orderbook);
        }
        _ => panic!("Order book snapshot data expected."),
    }

    // Update the order book 10 times
    for _i in 1..10 {
        match ws.next().await.unwrap() {
            Some(Data::OrderBookData(data))
            if data.action == OrderBookAction::Update => {
                for bid in &data.bids {
                    // Remove the bid
                    if bid.1 == Decimal::from(0) {
                        assert!(orderbook.bids.contains_key(&bid.0));
                        assert!(orderbook.bids.remove(&bid.0).is_some());
                    } else {
                        orderbook.bids.insert(bid.0, bid.1);
                    }
                }
                for ask in &data.asks {
                    // Remove the ask
                    if ask.1 == Decimal::from(0) {
                        assert!(orderbook.asks.contains_key(&ask.0));
                        assert!(orderbook.asks.remove(&ask.0).is_some());
                    } else {
                        orderbook.asks.insert(ask.0, ask.1);
                    }
                }
                // println!("{:#?}", orderbook);
            }
            _ => panic!("Order book update data expected."),
        }
    }
}
