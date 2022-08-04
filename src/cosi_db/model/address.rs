use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

use lipsum::lipsum_words_from_seed;

// cosi_db
use super::common::{COSICollection, Generator};
use crate::cosi_db::controller::common::get_connection;

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

#[async_trait]
impl COSICollection<Address> for Address {
    async fn get_collection() -> mongodb::Collection<Address> {
        get_connection().await.collection::<Address>("address")
    }
}

#[async_trait]
impl Generator<Address> for Address {
    async fn generate(size: u32) -> Vec<Address> {
        let mut result = Vec::new();
        let mut rng = thread_rng();

        // TODO: Generate optional.
        for i in 0..size {
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

        return result;
    }
}
