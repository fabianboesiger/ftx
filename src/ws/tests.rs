use super::*;
use crate::rest::{OrderStatus, Rest};
use anyhow::Result;
use dotenv::dotenv;
use rust_decimal_macros::dec;

async fn init_authenticated_ws() -> Ws {
    dotenv().ok();
    Ws::connect(Options::from_env())
        .await
        .expect("Connection failed.")
}
async fn init_unauthenticated_ws() -> Ws {
    dotenv().ok();
    Ws::connect(Options::default())
        .await
        .expect("Connection failed.")
}

#[allow(dead_code)]
async fn init_api() -> Result<Rest> {
    dotenv()?;

    Ok(Rest::new(Options::from_env()))
}

#[tokio::test]
async fn subscribe_unsubscribe_trades() {
    let mut ws = init_unauthenticated_ws().await;

    // Channels: BTC, ETH
    ws.subscribe(&[
        Channel::Trades("BTC-PERP".to_owned()),
        Channel::Trades("ETH-PERP".to_owned()),
    ])
    .await
    .expect("Subscribe failed");

    // Channels: BTC
    ws.unsubscribe(&[Channel::Trades("ETH-PERP".to_owned())])
        .await
        .expect("Unsubscribe failed");

    // Channels: BTC, LTC
    ws.subscribe(&[Channel::Trades("LTC-PERP".to_owned())])
        .await
        .expect("Subscribe failed");

    // Channels: None
    ws.unsubscribe_all().await.expect("Unsubscribe all failed");
}

#[tokio::test]
async fn trades() {
    let mut ws = init_unauthenticated_ws().await;

    ws.subscribe(&[Channel::Trades("BTC-PERP".to_owned())])
        .await
        .expect("Subscription failed.");

    match ws.next().await.unwrap() {
        Ok((Some(_), Data::Trade(..))) => {}
        _ => panic!("Trade data expected."),
    }

    ws.unsubscribe_all().await.expect("Unsubscribe failed");
}

#[tokio::test]
async fn order_book_update() {
    let mut ws = init_unauthenticated_ws().await;

    let symbol: Symbol = String::from("BTC-PERP");
    ws.subscribe(&[Channel::Orderbook(symbol.to_owned())])
        .await
        .expect("Subscription failed.");

    let mut orderbook = Orderbook::new(symbol);

    // The initial snapshot of the order book
    match ws.next().await.unwrap() {
        Ok((_, Data::OrderbookData(data))) if data.action == OrderbookAction::Partial => {
            orderbook.update(&data);
            assert!(orderbook.verify_checksum(data.checksum));
            // println!("{:#?}", orderbook);
        }
        _ => panic!("Order book snapshot data expected."),
    }

    // Update the order book 10 times
    for _i in 1..10 {
        match ws.next().await.unwrap() {
            Ok((_, Data::OrderbookData(data))) if data.action == OrderbookAction::Update => {
                // Check that removed orders are in the orderbook
                for bid in &data.bids {
                    if bid.1.is_zero() {
                        assert!(orderbook.bids.contains_key(&bid.0));
                    }
                }
                for ask in &data.asks {
                    if ask.1.is_zero() {
                        assert!(orderbook.asks.contains_key(&ask.0));
                    }
                }

                // Update the order book
                orderbook.update(&data);
                assert!(orderbook.verify_checksum(data.checksum));

                // Check that removed orders are no longer in the orderbook
                // Check that inserted orders have been updated correctly
                for bid in &data.bids {
                    if bid.1.is_zero() {
                        assert!(!orderbook.bids.contains_key(&bid.0));
                    } else {
                        assert_eq!(orderbook.bids.get(&bid.0), Some(&bid.1));
                    }
                }
                for ask in &data.asks {
                    if ask.1.is_zero() {
                        assert!(!orderbook.asks.contains_key(&ask.0));
                    } else {
                        assert_eq!(orderbook.asks.get(&ask.0), Some(&ask.1));
                    }
                }

                // println!("{:#?}", orderbook);
            }
            _ => panic!("Order book update data expected."),
        }
    }

    ws.unsubscribe_all().await.expect("Unsubscribe failed");
}

#[tokio::test]
async fn order_book_helpers() {
    let symbol: Symbol = String::from("SHIT-PERP");
    let mut ob = Orderbook::new(symbol);

    // All helpers should return None since there are no orders in the book
    assert_eq!(ob.bid_price(), None);
    assert_eq!(ob.ask_price(), None);
    assert_eq!(ob.mid_price(), None);
    assert_eq!(ob.best_bid(), None);
    assert_eq!(ob.best_ask(), None);
    assert_eq!(ob.best_bid_and_ask(), None);
    assert_eq!(ob.quote(Side::Buy, dec!(100)), None);

    // Asks
    ob.asks.insert(dec!(7), dec!(40));
    ob.asks.insert(dec!(6), dec!(30));
    ob.asks.insert(dec!(5), dec!(20));

    // Bids
    ob.bids.insert(dec!(4), dec!(5));
    ob.bids.insert(dec!(3), dec!(10));
    ob.bids.insert(dec!(2), dec!(15));

    assert_eq!(ob.bid_price().unwrap(), &dec!(4));
    assert_eq!(ob.ask_price().unwrap(), &dec!(5));
    assert_eq!(ob.mid_price().unwrap(), dec!(4.5));
    assert_eq!(ob.best_bid().unwrap(), (&dec!(4), &dec!(5)));
    assert_eq!(ob.best_ask().unwrap(), (&dec!(5), &dec!(20)));
    assert_eq!(
        ob.best_bid_and_ask().unwrap(),
        ((&dec!(4), &dec!(5)), (&dec!(5), &dec!(20)))
    );

    assert_eq!(ob.quote(Side::Buy, dec!(15)).unwrap(), dec!(5));
    assert_eq!(ob.quote(Side::Buy, dec!(20)).unwrap(), dec!(5));
    // 20 at $5, 5 at $6 = $5.2
    assert_eq!(ob.quote(Side::Buy, dec!(25)).unwrap(), dec!(5.2));
    // 20 at $5, 30 at $6 = $5.6
    assert_eq!(ob.quote(Side::Buy, dec!(50)).unwrap(), dec!(5.6));
    // 20 at $5, 30 at $6, 20 at $7 = 6
    assert_eq!(ob.quote(Side::Buy, dec!(70)).unwrap(), dec!(6));
    assert_eq!(ob.quote(Side::Buy, dec!(100)), None);

    // Likewise
    assert_eq!(ob.quote(Side::Sell, dec!(5)).unwrap(), dec!(4));
    assert_eq!(
        ob.quote(Side::Sell, dec!(7)).unwrap(),
        (dec!(20) + dec!(6)) / dec!(7)
    );
    assert_eq!(
        ob.quote(Side::Sell, dec!(15)).unwrap(),
        (dec!(20) + dec!(30)) / dec!(15)
    );
    assert_eq!(
        ob.quote(Side::Sell, dec!(17)).unwrap(),
        (dec!(20) + dec!(30) + dec!(4)) / dec!(17)
    );
    assert_eq!(
        ob.quote(Side::Sell, dec!(30)).unwrap(),
        (dec!(20) + dec!(30) + dec!(30)) / dec!(30)
    );
    assert_eq!(ob.quote(Side::Sell, dec!(100)), None);
}

#[tokio::test]
async fn order_book_checksum() {
    // BTC-PERP: Whole number prices, decimal and fractional quantities
    // ETH-PERP: Decimal prices, decimal and fractional quantities
    // ETH/BTC: Fractional prices, decimal and fractional quantities
    let symbols = vec!["BTC-PERP", "ETH-PERP", "ETH/BTC"];

    // Subscribe to each symbol and verify orderbook checksums for initial snapshot
    // and one orderbook update
    for symbol in symbols {
        let mut ws = init_unauthenticated_ws().await;

        ws.subscribe(&[Channel::Orderbook(symbol.to_string())])
            .await
            .expect("Subscription failed.");

        let mut orderbook = Orderbook::new(symbol.to_string());

        // Initial snapshot
        match ws.next().await.unwrap() {
            Ok((_, Data::OrderbookData(data))) if data.action == OrderbookAction::Partial => {
                orderbook.update(&data);
                assert!(orderbook.verify_checksum(data.checksum));
                // println!("{:#?}", orderbook);
            }
            _ => panic!("Order book snapshot data expected."),
        }

        // Orderbook update
        match ws.next().await.unwrap() {
            Ok((_, Data::OrderbookData(data))) if data.action == OrderbookAction::Update => {
                orderbook.update(&data);
                assert!(orderbook.verify_checksum(data.checksum));
            }
            _ => panic!("Order book update data expected."),
        }

        ws.unsubscribe_all().await.expect("Unsubscribe failed");
    }
}

#[tokio::test]
#[ignore]
async fn fills() {
    let mut ws = init_authenticated_ws().await;

    ws.subscribe(&[Channel::Fills])
        .await
        .expect("Subscription failed.");

    // A live test that buys 0.0001 BTC-PERP ($4 if BTC is at $40k)
    /*
    use crate::rest::{Side, OrderType};
    let api = init_api().await;
    api.place_order(
        "BTC-PERP",
        Side::Buy,
        None,
        OrderType::Market,
        dec!(0.0001),
        None,
        None,
        None,
        None,
    )
        .await
        .expect("Could not place order for testing purposes");
    match ws.next().await.unwrap() {
        Some(Data::Fill(..)) => {}
        _ => panic!("Fill data expected."),
    }
    */

    ws.unsubscribe_all().await.expect("Unsubscribe failed");
}

#[tokio::test]
async fn subscribe_authenticated_updates_on_unauthenticated_channel() {
    //     Trying to subscribe to the FILL or ORDER channels requires authentification
    //     and has to fail on an unauthenticated socket

    // TODO: match specific error
    let mut ws = init_unauthenticated_ws().await;
    ws.subscribe(&[Channel::Fills])
        .await
        .expect_err("Subscription should fail");
    ws.subscribe(&[Channel::Orders])
        .await
        .expect_err("Subscription should fail");
}

#[tokio::test]
#[ignore]
async fn orders() {
    let mut ws = init_authenticated_ws().await;

    ws.subscribe(&[Channel::Orders])
        .await
        .expect("Subscription failed.");

    // Manipulate some orders:
    // 1. Place order
    // 2. Modify order
    // 3. Cancel order
    // 4. Submit invalid post-only order
    crate::rest::tests::manipulate_orders().await;

    // Initial order placement
    let initial_order_placement = match ws.next().await.unwrap() {
        Ok((_, Data::Order(order))) => order,
        _ => panic!("Order data expected."),
    };
    // println!("{:?}", initial_order_placement);
    assert_eq!(OrderStatus::New, initial_order_placement.status);

    // Initial order cancelled during modification
    let initial_order_cancellation = match ws.next().await.unwrap() {
        Ok((_, Data::Order(order))) => order,
        _ => panic!("Order data expected."),
    };
    // println!("{:?}", initial_order_cancellation);
    assert_eq!(initial_order_placement.id, initial_order_cancellation.id);
    assert_eq!(OrderStatus::Closed, initial_order_cancellation.status);

    // Modified order placement
    let modified_order_placement = match ws.next().await.unwrap() {
        Ok((_, Data::Order(order))) => order,
        _ => panic!("Order data expected."),
    };
    // println!("{:?}", modified_order_placement);
    assert_ne!(initial_order_cancellation.id, modified_order_placement.id);
    assert_eq!(OrderStatus::New, modified_order_placement.status);

    // Modified order explicit cancellation
    let modified_order_cancellation = match ws.next().await.unwrap() {
        Ok((_, Data::Order(order))) => order,
        _ => panic!("Order data expected."),
    };
    // println!("{:?}", modified_order_cancellation);
    assert_eq!(modified_order_placement.id, modified_order_cancellation.id);
    assert_eq!(OrderStatus::Closed, modified_order_cancellation.status);

    // Rejected order placement
    let rejected_order_placement = match ws.next().await.unwrap() {
        Ok((_, Data::Order(order))) => order,
        _ => panic!("Order data expected."),
    };
    // println!("{:?}", rejected_order_placement);
    assert_ne!(modified_order_cancellation.id, rejected_order_placement.id);
    assert_eq!(OrderStatus::Closed, rejected_order_placement.status);

    ws.unsubscribe_all().await.expect("Unsubscribe failed");
}
