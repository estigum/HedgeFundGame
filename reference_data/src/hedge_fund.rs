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

#[test]
fn create_hedgefund(){
    let hedgefund = HedgeFund::new(12, String::from("Stigum Capital Management"), String::from("381 Rock Road, Glen Rock, NJ 07452"), String::from("631-219-3849"));
    assert_eq!(hedgefund.get_id(), 12);
    assert_eq!(hedgefund.get_name(), "Stigum Capital Management");

}