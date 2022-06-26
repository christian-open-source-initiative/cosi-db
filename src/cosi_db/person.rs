// Houses person to person interactions.
use chrono::NaiveDate;

pub enum Sex {
    Male,
    Female
}

pub struct Person {
    firstname: String,
    middlename: String,
    lastname: String,
    nicks: Vec<String>,
    dob: Option<NaiveDate>,
    age: Option<u8>,
    sex: Sex
}

