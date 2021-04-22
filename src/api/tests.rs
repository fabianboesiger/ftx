use super::*;
use dotenv::dotenv;
use std::env::var;

fn init_api() -> Api {
    dotenv().ok();
    Api::new(
        var("API_KEY").expect("API Key is not defined."), 
        var("API_SECRET").expect("API Secret is not defined."),
        None,
    )
}

fn read_only<T>(result: Result<T>) {
    match result {
        Err(Error::Api(error)) if error == String::from("Not allowed with read-only permissions") => (),
        _ => panic!("Expected read-only error.")
    }
}

#[tokio::test]
async fn get_subaccounts() {
    init_api().get_subaccounts().await.unwrap();
}

#[tokio::test]
async fn create_subaccount() {
    read_only(init_api().create_subaccount("Bot").await);
}

#[tokio::test]
async fn change_subaccount_name() {
    read_only(init_api().change_subaccount_name("Bot", "Bot").await);
}

#[tokio::test]
async fn delete_subaccount() {
    read_only(init_api().delete_subaccount("Bot").await);
}

#[tokio::test]
async fn get_subaccount_balances() {
    init_api().get_subaccount_balances("Bot").await.unwrap_err();
}

#[tokio::test]
async fn transfer_between_subaccounts() {
    init_api().transfer_between_subaccounts("BTC", Decimal::zero(), "Source", "Destination").await.unwrap_err();
}

#[tokio::test]
async fn get_markets() {
    init_api().get_markets().await.unwrap();
}

#[tokio::test]
async fn get_market() {
    init_api().get_market("BTC/USD").await.unwrap();
}

#[tokio::test]
async fn get_orderbook() {
    init_api().get_orderbook("BTC/USD", None).await.unwrap();
    init_api().get_orderbook("BTC/USD", Some(50)).await.unwrap();
}

#[tokio::test]
async fn get_trades() {
    init_api().get_trades("BTC/USD", None, None, None).await.unwrap();
}