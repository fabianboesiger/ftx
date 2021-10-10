use dotenv::dotenv;
use ftx::{
    options::Options,
    rest::{GetAccountRequest, GetPositionsRequest, Rest},
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api = Rest::new(Options::from_env());
    println!("Account:");
    println!("{:#?}", api.request(GetAccountRequest).await.unwrap());
    println!("Positions:");
    println!("{:#?}", api.request(GetPositionsRequest).await.unwrap());
}
