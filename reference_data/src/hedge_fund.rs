use chrono::{Datelike, Timelike, Utc, DateTime};
use std::fmt;

pub struct HedgeFund {
    id : i32,
    name : String,
    address : String,
    phone_number : String,
}

impl HedgeFund {
    pub fn new(id : i32, name : String, address : String, phone_number : String) -> Self {
        HedgeFund {
            id,
            name,
            address,
            phone_number
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_address(&self) -> &str {
        self.address.as_str()
    }

    pub fn get_phone_number(&self) -> &str {
        self.phone_number.as_str()
    }
}

impl fmt::Display for HedgeFund {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Hedge Fund Info\nId: {} \nName: {} \nAddress: {} \nPhone#: {}", self.get_id(), self.get_name(), self.get_address(), self.get_phone_number())
    }
}

pub enum Side {
    Buy,
    Sell
}

pub struct Transaction {
    hedge_fund_id : i32,
    symbol : String,
    side : Side,
    quantity : f32,
    price : f32,
    transact_time : DateTime<Utc>,
}

impl Transaction {
    pub fn new(hedge_fund_id : i32, symbol : String, side : Side, quantity : f32, price : f32) -> Self {
        Transaction {
            hedge_fund_id,
            symbol,
            side,
            quantity,
            price,
            transact_time : Utc::now(),
        }
    }

    pub fn get_quantity(&self) -> f32 {
        self.quantity
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn get_hedge_fund_id(&self) -> i32 {
        self.hedge_fund_id
    }

    pub fn get_side(&self) -> &Side {
        &self.side
    }

    pub fn get_symbol(&self) -> &str {
        self.symbol.as_str()
    }

    pub fn get_transact_time(&self) -> String {
        let (is_pm, hour) = self.transact_time.hour12();
        let (is_common_era, year) = self.transact_time.year_ce();
        format!("{}/{}/{}-{} {} {}", self.transact_time.month(), self.transact_time.day(), year ,hour, self.transact_time.minute(), self.transact_time.second())
    }
}
pub struct Order {
    hedge_fund_id : i32,
    symbol : String,
    side : Side,
    quantity : f32,
    price : f32,
    transact_time : DateTime<Utc>,
}
impl Order {
    pub fn new(hedge_fund_id : i32, symbol : String, side : Side, quantity : f32, price : f32) -> Self {
        Order {
            hedge_fund_id,
            symbol,
            side,
            quantity,
            price,
            transact_time : Utc::now(),
        }
    }

    pub fn get_quantity(&self) -> f32 {
        self.quantity
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn get_hedge_fund_id(&self) -> i32 {
        self.hedge_fund_id
    }

    pub fn get_side(&self) -> &Side {
        &self.side
    }

    pub fn get_symbol(&self) -> &str {
        self.symbol.as_str()
    }

    pub fn get_transact_time(&self) -> String {
        let (is_pm, hour) = self.transact_time.hour12();
        let (is_common_era, year) = self.transact_time.year_ce();
        format!("{}/{}/{}-{} {} {}", self.transact_time.month(), self.transact_time.day(), year ,hour, self.transact_time.minute(), self.transact_time.second())
    }    
}


#[test]
fn create_hedgefund(){
    let hedgefund = HedgeFund::new(12, String::from("Stigum Capital Management"), String::from("381 Rock Road, Glen Rock, NJ 07452"), String::from("631-219-3849"));
    assert_eq!(hedgefund.get_id(), 12);
    assert_eq!(hedgefund.get_name(), "Stigum Capital Management");

}

#[test]
fn create_hedgefund_transaction() {
    let order = Transaction::new(12, String::from("GBP/USD"), Side::Buy, 125000.00, 1.26);
    assert_eq!(order.get_hedge_fund_id(), 12);
    assert_eq!(order.get_symbol(), "GBP/USD");
}