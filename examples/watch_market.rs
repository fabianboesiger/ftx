use dotenv::dotenv;
use ftx::ws::{
    model::{Channel, Data, Orderbook},
    Result, Ws,
};
use std::env::var;
use std::io;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut websocket = Ws::connect(
        var("API_KEY").expect("API Key is not defined."),
        var("API_SECRET").expect("API Secret is not defined."),
        var("SUBACCOUNT").ok(),
    )
    .await?;

    let market = String::from("BTC-PERP");
    let mut orderbook = Orderbook::new(market.to_owned());

    websocket
        .subscribe(vec![
            Channel::Trades(market.to_owned()),
            Channel::Orderbook(market.to_owned()),
        ])
        .await?;

    loop {
        let data = websocket.next().await?.expect("No data received");

        match data {
            Data::Trade(trade) => {
                println!(
                    "\n{:?} {} {} at {} - liquidation = {}",
                    trade.side, trade.size, market, trade.price, trade.liquidation
                );
            }
            Data::OrderbookData(orderbook_data) => {
                orderbook.update(&orderbook_data);
                print!("."); // To signify orderbook update
                io::stdout().flush().unwrap(); // Emits the output immediately
            }
            _ => panic!("Unexpected data type"),
        }
    }
}
