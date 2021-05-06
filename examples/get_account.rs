use ftx::api::Api;
use std::env::var;

#[tokio::main]
async fn main() {
    let api = Api::new(
        var("API_KEY").expect("API key not defined"),
        var("API_SECRET").expect("API secret not defined"),
        None,
    );
    println!("Account:");
    println!("{:#?}", api.get_account().await.unwrap());
    println!("Positions:");
    println!("{:#?}", api.get_positions().await.unwrap());
}
