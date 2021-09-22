use dotenv::dotenv;
use ftx::{options::Options, rest::Rest};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api = Rest::new(Options::from_env());
    println!("Account:");
    println!("{:#?}", api.get_account().await.unwrap());
    println!("Positions:");
    println!("{:#?}", api.get_positions().await.unwrap());
}
