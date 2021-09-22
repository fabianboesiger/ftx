use dotenv::dotenv;
use ftx::{options::Options, rest::Rest};
use std::env::var;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api = Rest::new(
        Options::default()
            .authenticate(
                var("API_KEY").expect("API Key is not defined."),
                var("API_SECRET").expect("API Secret is not defined."),
            )
            .subaccount_optional(var("SUBACCOUNT").ok()),
    );
    println!("Account:");
    println!("{:#?}", api.get_account().await.unwrap());
    println!("Positions:");
    println!("{:#?}", api.get_positions().await.unwrap());
}
