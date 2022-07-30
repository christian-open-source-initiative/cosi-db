use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Address {
    pub line_one: String,
    pub line_two: String,
    pub line_three: String,
    pub city: String,
    pub region: String,
    pub postal_code: Option<String>,
    pub county: Option<String>,
    pub country: Option<String>,
}
