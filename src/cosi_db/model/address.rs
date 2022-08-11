use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use rand::{thread_rng, Rng};

use lipsum::lipsum_words_from_seed;
use std::default::Default;

use rocket::form::FromForm;

// cosi_db
use super::common::{COSICollection, Generator};
use crate::cosi_db::controller::common::get_connection;
use crate::cosi_db::errors::COSIResult;
use crate::cosi_db::model::common::COSIForm;

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
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

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct AddressForm {
    pub line_one: Option<String>,
    pub line_two: Option<String>,
    pub line_three: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<Option<String>>,
    pub county: Option<Option<String>>,
    pub country: Option<Option<String>>,
}
impl COSIForm for AddressForm {}

impl Default for Address {
    fn default() -> Self {
        Address {
            line_one: "line_one".to_string(),
            line_two: "line_two".to_string(),
            line_three: "line_three".to_string(),
            city: "city".to_string(),
            region: "region".to_string(),
            postal_code: None,
            county: None,
            country: None,
        }
    }
}

#[async_trait]
impl COSICollection<'_, Address, Address, AddressForm> for Address {
    fn get_table_name() -> String {
        return "address".to_string();
    }

    async fn get_collection() -> mongodb::Collection<Address> {
        get_connection().await.collection::<Address>("address")
    }
}

#[async_trait]
impl Generator<Address> for Address {
    async fn generate(size: u32) -> COSIResult<Vec<Address>> {
        let mut result = Vec::new();
        let mut rng = thread_rng();

        // TODO: Generate optional.
        for _ in 0..size {
            let seed: u64 = rng.gen_range(0, 2_u64.pow(32));
            result.push(Address {
                line_one: lipsum_words_from_seed(8, seed),
                line_two: lipsum_words_from_seed(8, seed),
                line_three: lipsum_words_from_seed(8, seed),
                city: lipsum_words_from_seed(1, seed),
                region: lipsum_words_from_seed(1, seed),
                postal_code: Some(lipsum_words_from_seed(1, seed)),
                county: Some(lipsum_words_from_seed(2, seed)),
                country: Some(lipsum_words_from_seed(3, seed)),
            });
        }

        return Ok(result);
    }
}
