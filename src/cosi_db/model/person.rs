// Houses person to person interactions.
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Sex {
    Male,
    Female,
    Undefined,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub nicks: Vec<String>,
    pub dob: Option<NaiveDate>,
    pub age: Option<u8>,
    pub sex: Sex,
}
