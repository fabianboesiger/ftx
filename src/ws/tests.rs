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
            orderbook.update(&data);
            // println!("{:#?}", orderbook);
        }
        _ => panic!("Order book snapshot data expected."),
    }

    // Update the order book 10 times
    for _i in 1..10 {
        match ws.next().await.unwrap() {
            Some(Data::OrderBookData(data))
            if data.action == OrderBookAction::Update => {

                // Check that cancelled orders are in the orderbook
                for bid in &data.bids {
                    if bid.1 == Decimal::from(0) {
                        assert!(orderbook.bids.contains_key(&bid.0));
                    }
                }
                for ask in &data.asks {
                    if ask.1 == Decimal::from(0) {
                        assert!(orderbook.asks.contains_key(&ask.0));
                    }
                }

                // Update the order book
                orderbook.update(&data);

                // Check that cancelled orders are no longer in the orderbook
                // Check that inserted orders have been updated correctly
                for bid in &data.bids {
                    if bid.1 == Decimal::from(0) {
                        assert_eq!(orderbook.bids.contains_key(&bid.0), false);
                    } else {
                        assert_eq!(orderbook.bids.get(&bid.0), Some(&bid.1));
                    }
                }
                for ask in &data.asks {
                    if ask.1 == Decimal::from(0) {
                        assert_eq!(orderbook.asks.contains_key(&ask.0), false);
                    } else {
                        assert_eq!(orderbook.asks.get(&ask.0), Some(&ask.1));
                    }
                }

                // println!("{:#?}", orderbook);
            }
            _ => panic!("Order book update data expected."),
        }
    }
}
