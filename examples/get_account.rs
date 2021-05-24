use dotenv::dotenv;
use ftx::rest::Rest;
use std::env::var;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api = Rest::new(
        var("API_KEY").expect("API key not defined"),
        var("API_SECRET").expect("API secret not defined"),
        var("SUBACCOUNT").ok(),
    );
    println!("Account:");
    println!("{:#?}", api.get_account().await.unwrap());
    println!("Positions:");
    println!("{:#?}", api.get_positions().await.unwrap());
}
