use {
    dotenv::dotenv,
    ftx::{
        options::Options,
        rest::{
            GetLendingInfo, GetLendingRates, GetMyLendingHistory, LendingInfo, Rest,
            SubmitLendingOffer,
        },
    },
    std::env,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api = Rest::new(Options::from_env());

    let lending_info = api.request(GetLendingInfo {}).await.unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{:#?}", lending_info);

        let lending_rates = api.request(GetLendingRates {}).await.unwrap();
        println!("{:#?}", lending_rates);

        let lending_history = api.request(GetMyLendingHistory::default()).await.unwrap();
        println!("{:#?}", lending_history);
    } else {
        let coin = &args[1];

        for LendingInfo {
            lendable, min_rate, ..
        } in lending_info
            .iter()
            .filter(|lending_info| lending_info.coin == *coin)
        {
            let size = lendable.floor();
            println!("Submitting lending offer for {}: {}", coin, size);
            api.request(SubmitLendingOffer {
                coin: coin.clone(),
                size,
                rate: min_rate.unwrap_or_default(),
            })
            .await
            .unwrap();
        }
    }
}
