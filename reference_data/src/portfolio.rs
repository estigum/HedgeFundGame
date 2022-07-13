use std::collections::HashMap;
use crate::hedge_fund::{Transaction,Side};

pub struct Position {
    currency : String,
    amount : f32,
}

impl Position {
    pub fn new(currency : String, amount : f32) -> Self {
        Position {
            currency,
            amount,
        }
    }

    pub fn get_currency(&self) -> &str {
        self.currency.as_str()
    }

    pub fn get_amount(&self) -> f32 {
        self.amount
    }
}

pub struct Portfolio {
    hedge_fund_id : i32,
    positions : HashMap<String,Position>,
}

impl Portfolio {
    pub fn new(hedge_fund_id : i32) -> Self {
        Portfolio {
            hedge_fund_id,
            positions : HashMap::new(),
        }
    }

    pub fn add_postion(&mut self, position : Position) -> Result<bool, &'static str>
    {
        if !self.positions.contains_key(position.get_currency()) {
            self.positions.insert(position.get_currency().to_string(), position);
            Ok(true)
        }
        else
        {
            Err("Position record already exist for currency. Can't be added")
        }
    }
    
    pub fn add_transaction(&mut self, transaction : &Transaction) -> Result<bool, &'static str> {
        if self.positions.contains_key(transaction.get_symbol()) {
            let position = self.positions.get_mut(transaction.get_symbol());
            if let Some(p) = position {
                match transaction.get_side() {
                    Side::Buy => p.amount += transaction.get_quantity(),
                    Side::Sell => {
                        if transaction.get_quantity() < p.amount {
                            p.amount -= transaction.get_quantity()
                        }
                        else {
                            return Err("Not enough quanity remaining")
                        }
                    }
                }
                return Ok(true);
            }
        }
        Err("Not able to get postion")
    }

    pub fn get_position(&self, currency : &str) -> Option<f32> {
        if self.positions.contains_key(currency) {
            let position = self.positions.get(currency);
            if let Some(p) = position {
                return Some(p.amount);
            }
        }
        None       
    }
}

#[test]
fn test_add_position() {

    let position = Position::new(String::from("USD"), 1000000.0);
    let mut portfolio = Portfolio::new(21);
    let currency = String::from("USD");
    portfolio.add_postion(position);
    assert_eq!(portfolio.get_position(&currency), Some(1000000.0));
}

#[test]
fn test_add_position_twice() {
    let position = Position::new(String::from("USD"), 1000000.0);
    let position2 = Position::new(String::from("USD"), 1000000.0);
    let mut portfolio = Portfolio::new(21);
    portfolio.add_postion(position);
    let result = portfolio.add_postion(position2);
    match result {
        Ok(val) => panic!("Should not happen"),
        Err(e) => assert_eq!(e, "Position record already exist for currency. Can't be added")
    }
}

#[test]
fn test_add_buy_transaction() {
    let position = Position::new(String::from("USD"), 1000000.0);
    let mut portfolio = Portfolio::new(21);
    portfolio.add_postion(position);
    let transaction = Transaction::new(21, String::from("USD"), Side::Buy, 200000.0, 1.0);
    let result = portfolio.add_transaction(&transaction);
    match result {
        Ok(val) => assert_eq!(val, true),
        Err(_) => panic!("This test case did not work")
    }
    let currency = String::from("USD");
    assert_eq!(portfolio.get_position(&currency), Some(1200000.0));  
}

#[test]
fn test_add_sell_transaction() {
    let position = Position::new(String::from("USD"), 1000000.0);
    let mut portfolio = Portfolio::new(21);
    portfolio.add_postion(position);
    let transaction = Transaction::new(21, String::from("USD"), Side::Sell, 200000.0, 1.0);
    let result = portfolio.add_transaction(&transaction);
    match result {
        Ok(val) => assert_eq!(val, true),
        Err(_) => panic!("This test case did not work")
    }
    let currency = String::from("USD");
    assert_eq!(portfolio.get_position(&currency), Some(800000.0));  
}
#[test]
fn test_add_sell_transactions_not_enough_quantity() {
    let position = Position::new(String::from("USD"), 1000000.0);
    let mut portfolio = Portfolio::new(21);
    portfolio.add_postion(position);
    let transaction = Transaction::new(21, String::from("USD"), Side::Sell, 200000.0, 1.0);
    let result = portfolio.add_transaction(&transaction);
    match result {
        Ok(val) => assert_eq!(val, true),
        Err(_) => panic!("This test case did not work")
    }
    let currency = String::from("USD");
    assert_eq!(portfolio.get_position(&currency), Some(800000.0));  
    let transaction2 = Transaction::new(21, String::from("USD"), Side::Sell, 2000000.0, 1.0);
    let result = portfolio.add_transaction(&transaction2);
    match result {
        Ok(val) => panic!("This test case did not work"),
        Err(e) => assert_eq!(e, "Not enough quanity remaining") 
    } 
}
