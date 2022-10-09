use dotenvy::dotenv;
use ftx::{
    options::Options,
    rest::{GetAccount, GetPositions, Rest},
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api = Rest::new(Options::from_env());
    println!("Account:");
    println!("{:#?}", api.request(GetAccount {}).await.unwrap());
    println!("Positions:");
    println!("{:#?}", api.request(GetPositions {}).await.unwrap());
}
