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