use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct Address {
    pub address1: String,
    pub pincode: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct User {
    pub name: String,
    
    pub email: String,
    pub address: Vec<Address>,
}
