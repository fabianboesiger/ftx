use dotenv::dotenv;
use ftx::{
    options::Options,
    rest::{GetMarketRequest, Rest, Result},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api = Rest::new(Options::from_env());

    let price = api.request(GetMarketRequest::new("BTC/USD")).await?.price;
    println!("1 BTC is worth {} USD.", price);

    Ok(())
}
