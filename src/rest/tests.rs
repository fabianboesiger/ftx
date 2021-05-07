use super::*;
use dotenv::dotenv;
use std::env::var;

async fn init_api() -> Rest {
    dotenv().ok();

    let api = Rest::new(
        var("API_KEY").expect("API Key is not defined."),
        var("API_SECRET").expect("API Secret is not defined."),
        None,
    );

    read_only(api.create_subaccount("Bot").await);

    api
}

fn read_only<T>(result: Result<T>) {
    match result {
        Err(Error::Api(error))
            if error == String::from("Not allowed with read-only permissions") =>
        {
            ()
        }
        _ => panic!("Expected read-only subaccount."),
    }
}

#[tokio::test]
async fn get_subaccounts() {
    init_api().await.get_subaccounts().await.unwrap();
}

#[tokio::test]
async fn create_subaccount() {
    read_only(init_api().await.create_subaccount("Bot").await);
}

#[tokio::test]
async fn change_subaccount_name() {
    read_only(init_api().await.change_subaccount_name("Bot", "Bot").await);
}

#[tokio::test]
async fn delete_subaccount() {
    read_only(init_api().await.delete_subaccount("Bot").await);
}

#[tokio::test]
async fn get_subaccount_balances() {
    init_api().await.get_subaccount_balances("Bot").await.unwrap_err();
}

#[tokio::test]
async fn transfer_between_subaccounts() {
    init_api()
        .await
        .transfer_between_subaccounts("BTC", Decimal::zero(), "Source", "Destination")
        .await
        .unwrap_err();
}

#[tokio::test]
async fn get_markets() {
    init_api().await.get_markets().await.unwrap();
}

#[tokio::test]
async fn get_market() {
    init_api().await.get_market("BTC/USD").await.unwrap();
}

#[tokio::test]
async fn get_orderbook() {
    init_api().await.get_orderbook("BTC/USD", None).await.unwrap();
    init_api().await.get_orderbook("BTC/USD", Some(50)).await.unwrap();
}

#[tokio::test]
async fn get_trades() {
    init_api()
        .await
        .get_trades("BTC/USD", None, None, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn get_historical_prices() {
    init_api()
        .await
        .get_historical_prices("BTC/USD", 300, None, None, None)
        .await
        .unwrap();
}


#[tokio::test]
async fn get_futures() {
    init_api().await.get_futures().await.unwrap();
}

#[tokio::test]
async fn get_future() {
    init_api().await.get_future("BTC-PERP").await.unwrap();
}