use std::collections::HashMap;
use reference_data::hedge_fund::{HedgeFund};
use reference_data::portfolio::{Portfolio};

pub struct PortfolioManager {
    latest_hedge_fund_id : i32,
    hedge_funds : HashMap<i32, HedgeFund>,
    portfolios : HashMap<i32, Portfolio>,
}

impl PortfolioManager {
    pub fn new() -> Self {
        PortfolioManager {
            latest_hedge_fund_id : 0,
            hedge_funds : HashMap::new(),
            portfolios : HashMap::new(),
        }
    }
    pub fn add_new_hedge_fund(&mut self, name : String, address : String, phone_number : String) ->Result<bool, &'static str>{
        if let Some(hedge_fund_id) = self.get_hedge_fund_id(&name) {
            Err("A hedge fund for name exist")
        }
        else {
            self.latest_hedge_fund_id += 1;
            let hedge_fund = HedgeFund::new(self.latest_hedge_fund_id, name.clone(), address.clone(), phone_number.clone());
            self.hedge_funds.insert(self.latest_hedge_fund_id, hedge_fund);
            Ok(true)    
        }
    }

    pub fn get_hedge_fund(&self, hedge_fund_id : i32) -> Result<&HedgeFund, &'static str> {
        if let Some(hedge_fund) = self.hedge_funds.get(&hedge_fund_id) {
            Ok(&hedge_fund)
        }
        else
        {
            Err("Not able to find hedge fund")
        }
    }

    pub fn get_hedge_fund_id(&self, name : &str) -> Option<i32> {
        for (key, value) in &self.hedge_funds {
            if value.get_name() == name {
                return Some(value.get_id());    
            }
        }
        None
    }

    pub fn add_portfolio(&mut self, hedge_fund_id : i32) -> Result<bool, &'static str> {
        if !self.hedge_funds.contains_key(&hedge_fund_id) {
            return Err("Hedge Fund does not exist")
        }
        if self.portfolios.contains_key(&hedge_fund_id) {
            Err("Portfolio already Exist for that Hedge Fund")
        }
        else{
            let portfolio = Portfolio::new(hedge_fund_id);
            self.portfolios.insert(hedge_fund_id, portfolio);
            Ok(true)
        }
    }

}

#[test]
fn test_adding_hedge_fund()
{
    let mut portfolio_manager = PortfolioManager::new();
    portfolio_manager.add_new_hedge_fund(String::from("Stigum Investments"), String::from("381 Rock Road"), String::from("631-219-3849"));
    let result = portfolio_manager.get_hedge_fund(1);
    match  result {
        Ok(hedge_fund) => {
           assert_eq!(hedge_fund.get_name(),"Stigum Investments");         
        },
        Err(_) => panic!("Error running case")
    }
}

#[test]
fn test_adding_hedge_fund_twice()
{
    let mut portfolio_manager = PortfolioManager::new();
    portfolio_manager.add_new_hedge_fund(String::from("Stigum Investments"), String::from("381 Rock Road"), String::from("631-219-3849"));
    let result = portfolio_manager.add_new_hedge_fund(String::from("Stigum Investments"), String::from("381 Rock Road"), String::from("631-219-3849"));
    match  result {
        Ok(_) => panic!("Error running case"),
        Err(e) => assert_eq!(e,"A hedge fund for name exist")
    }
}

#[test]
fn test_hedge_fund_exist()
{
    let mut portfolio_manager = PortfolioManager::new();
    portfolio_manager.add_new_hedge_fund(String::from("Stigum Investments"), String::from("381 Rock Road"), String::from("631-219-3849"));
    let result = portfolio_manager.get_hedge_fund(1);
    match  result {
        Ok(hedge_fund) => {
           assert_eq!(hedge_fund.get_name(),"Stigum Investments");         
        },
        Err(_) => panic!("Error running case")
    }
    let name = "Stigum Investments";
    if let Some(id) = portfolio_manager.get_hedge_fund_id(&name)
    {
        assert_eq!(id,1);
    }
    else
    {
        panic!("This case did not work");
    }

}

#[test]
fn add_portfolio(){
    let mut portfolio_manager = PortfolioManager::new();
    portfolio_manager.add_new_hedge_fund(String::from("Stigum Investments"), String::from("381 Rock Road"), String::from("631-219-3849"));
    let result = portfolio_manager.add_portfolio(1);
    match  result {
        Ok(value) => {
           assert_eq!(value, true);         
        },
        Err(_) => panic!("Error running case")
    }
}

#[test]
fn add_portfolio_non_existing_hedge_fund() {
    let mut portfolio_manager = PortfolioManager::new();
    portfolio_manager.add_new_hedge_fund(String::from("Stigum Investments"), String::from("381 Rock Road"), String::from("631-219-3849"));
    let result = portfolio_manager.add_portfolio(3);
    match  result {
        Ok(_) => {
            panic!("Error running case")    
        },
        Err(e) => assert_eq!(e, "Hedge Fund does not exist")
    }
}

#[test]
fn test_adding_portfolio_twice_for_hedgefund()
{
    let mut portfolio_manager = PortfolioManager::new();
    portfolio_manager.add_new_hedge_fund(String::from("Stigum Investments"), String::from("381 Rock Road"), String::from("631-219-3849"));
    let result = portfolio_manager.add_portfolio(1);
    match  result {
        Ok(value) => {
           assert_eq!(value, true);         
        },
        Err(_) => panic!("Error running case")
    }
    let result = portfolio_manager.add_portfolio(1);
    match  result {
        Ok(_) => {
            panic!("Error running case")       
        },
        Err(e) => assert_eq!(e,"Portfolio already Exist for that Hedge Fund")
    }
}