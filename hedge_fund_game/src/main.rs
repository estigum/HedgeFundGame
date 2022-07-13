use reference_data::{hedge_fund::HedgeFund, hedge_fund::Transaction};
use market_data::{coinbase};

#[tokio::main]
async fn main() ->Result<(), reqwest::Error> {
    println!("Hello, world!");
    let hedge_fund = HedgeFund::new(12,String::from("Stigum Capital"),String::from("381 Rock Road, Glen Rock, NJ 07452"), String::from("631-219-3849"));
    println!("Hedge Fund id is {}", hedge_fund.get_id());
    println!("{}", hedge_fund);
    
    let currency_pair = "ETH-USD";
    let result = coinbase::get_currencypair(&currency_pair).await;
    match result {
        Ok(value) => println!("{}-{} Price: {}", value.get_base(), value.get_currency(), value.get_amount()),
        Err(_) =>println!("Error getting price for currency pair")
    }
    let btc = "BTC-USD";
    if let Some(price) = get_price(&btc).await {
        println!("{} price is {}", btc, price);
    }
    Ok(())
}

async fn get_price(currency_pair : &str) ->Option<f64> {
    let result = coinbase::get_currencypair(&currency_pair).await;
    match result {
        Ok(value) => Some(value.get_amount()),
        Err(_) =>None
    }
}
