use super::*;
use dotenv::dotenv;
use std::env::var;

async fn init_api() -> Rest {
    dotenv().ok();

    let subaccount = var("SUBACCOUNT").ok();

    let api = Rest::new(
        var("API_KEY").expect("API Key is not defined."),
        var("API_SECRET").expect("API Secret is not defined."),
        var("SUBACCOUNT").ok(),
    );

    // Test create subaccount only if credentials are account-wide
    if let None = subaccount {
        read_only(api.create_subaccount("Bot").await);
    }

    api
}

fn read_only<T>(result: Result<T>) {
    match result {
        Err(Error::Api(error)) if error == *"Not allowed with read-only permissions" => {}
        _ => panic!("Expected read-only subaccount."),
    }
}

#[tokio::test]
async fn get_subaccounts() {
    let rest = init_api().await;
    if let None = rest.subaccount { // Test only if credentials are account-wide
        rest.get_subaccounts().await.unwrap();
    }
}

#[tokio::test]
async fn create_subaccount() {
    let rest = init_api().await;
    if let None = rest.subaccount { // Test only if credentials are account-wide
        read_only(rest.create_subaccount("Bot").await);
    }
}

#[tokio::test]
async fn change_subaccount_name() {
    let rest = init_api().await;
    if let None = rest.subaccount { // Test only if credentials are account-wide
        read_only(rest.change_subaccount_name("Bot", "Bot").await);
    }
}

#[tokio::test]
async fn delete_subaccount() {
    let rest = init_api().await;
    if let None = rest.subaccount { // Test only if credentials are account-wide
        read_only(rest.delete_subaccount("Bot").await);
    }
}

#[tokio::test]
async fn get_subaccount_balances() {
    let rest = init_api().await;
    // Test using given subaccount otherwise use "Bot"
    let subaccount = match &rest.subaccount {
        None => "Bot",
        Some(sub) => sub,
    };
    rest
        .get_subaccount_balances(&subaccount)
        .await
        .unwrap_err();
}

#[tokio::test]
async fn transfer_between_subaccounts() {
    let rest = init_api().await;
    if let None = rest.subaccount { // Test only if credentials are account-wide
        rest
            .transfer_between_subaccounts("BTC", Decimal::zero(), "Source", "Destination")
            .await
            .unwrap_err();
    }
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
    init_api()
        .await
        .get_orderbook("BTC/USD", None)
        .await
        .unwrap();
    init_api()
        .await
        .get_orderbook("BTC/USD", Some(50))
        .await
        .unwrap();
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

#[tokio::test]
async fn account_deserialization() {
    // Sanitized response from FTX API.
    let json = r#"{
        "backstopProvider":false,
        "chargeInterestOnNegativeUsd":false,
        "collateral":123,
        "freeCollateral":0.0,
        "initialMarginRequirement":0.01,
        "leverage":1.0,
        "liquidating":false,
        "maintenanceMarginRequirement":0.03,
        "makerFee":-5e-6,
        "marginFraction":0.01,
        "openMarginFraction":0.01,
        "positionLimit":null,
        "positionLimitUsed":null,
        "positions":[{
            "collateralUsed":123.45,
            "cost":-123.45,
            "entryPrice":1.23,
            "estimatedLiquidationPrice":1.23,
            "future":"FOOBAR",
            "initialMarginRequirement":0.1,
            "longOrderSize":0.0,
            "maintenanceMarginRequirement":0.01,
            "netSize":12345,
            "openSize":12345,
            "realizedPnl":12345,
            "shortOrderSize":0.0,
            "side":"sell",
            "size":12345,
            "unrealizedPnl":0.0
        }],
        "spotLendingEnabled":true,
        "spotMarginEnabled":true,
        "takerFee":0.0001,
        "totalAccountValue":123.1,
        "totalPositionSize":123.1,
        "useFttCollateral":true,
        "username":"foo@example.com"
    }"#;
    let _account: Account = serde_json::from_str(json).unwrap();
}
