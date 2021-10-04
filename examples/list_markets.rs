use dotenv::dotenv;
use ftx::{
    options::Options,
    rest::{Rest, Result},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api = Rest::new(Options::from_env());

    for market in api.get_markets().await? {
        println!(
            "Market {} had ${} volume in the last 24 hours.",
            market.name, market.volume_usd24h
        );
    }

    Ok(())
}
