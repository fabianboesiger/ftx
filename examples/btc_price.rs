use dotenv::dotenv;
use ftx::{
    options::Options,
    rest::{Rest, Result},
};
use std::env::var;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api = Rest::new(
        Options::default()
            .authenticate(
                var("API_KEY").expect("API Key is not defined."),
                var("API_SECRET").expect("API Secret is not defined."),
            )
            .subaccount_optional(var("SUBACCOUNT").ok()),
    );

    let price = api.get_market("BTC/USD").await?.price;
    println!("1 BTC is worth {} USD.", price);

    Ok(())
}
