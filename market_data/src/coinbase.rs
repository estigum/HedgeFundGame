use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinbaseCurrency {
    base : String,
    currency : String,
    amount : String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CoinBase {
    data : CoinbaseCurrency,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CoinBaseData {
    data : Vec<CoinBaseProducts>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinBaseProducts {
    pub id : String,
    base_currency : String,
    quote_currency : String,
    quote_increment : String,
    base_increment : String,
    display_name : String,
    min_market_funds : String,
    margin_enabled : bool,
    post_only : bool,
    limit_only : bool,
    cancel_only : bool,
    status : String,
    status_message : String,
    trading_disabled : bool,
    fx_stablecoin : bool,
    max_slippage_percentage : String,
    auction_mode : bool,
}

impl CoinBase {
    pub fn get_base(&self) -> &str {
        self.data.base.as_str()
    }
    pub fn get_currency(&self) -> &str {
        self.data.currency.as_str()
    }
    pub fn get_amount(&self) -> f64 {
        self.data.amount.parse().unwrap_or_default()
    }

}

pub async fn get_currencypair(currency_pair : &str) -> Result<CoinBase, reqwest::Error> {
    let url = format!("https://api.coinbase.com/v2/prices/{}/spot", currency_pair);
    let data : CoinBase = reqwest::Client::new()
    .get(url)
    .send()
    .await?
    .json()
    .await?;
    Ok(data)
}

pub async fn get_products() ->Result<Vec<CoinBaseProducts>, reqwest::Error> {
    let url = format!("https://api.exchange.coinbase.com/products");
    let data : Vec<CoinBaseProducts> = reqwest::Client::new()
    .get("https://api.exchange.coinbase.com/products")
    .header("User-Agent","application/json")
    .send()
    .await?
    .json()
    .await?;
    Ok(data)
}

#[tokio::test]
async fn test_currencypair()
{
    let currency_pair = "ETH-USD";
    let result = get_currencypair(&currency_pair).await;
    match result {
        Ok(val) => assert_eq!(val.get_base(), "ETH"),
        Err(_) => panic!("test_currencypair() FAILED!!!!")
    }
}

#[tokio::test]
async fn test_amount_greater_than_zero()
{
    let currency_pair = "ETH-USD";
    let result = get_currencypair(&currency_pair).await;
    match result {
        Ok(val) => assert!(val.get_amount() > 0.0),
        Err(_) => panic!("test_amount_greater_than_zero() FAILED!!!!")
    }
}