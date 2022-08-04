use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

// cosi_db
use crate::cosi_db::connection::{CosiDB, MongoConnection};
use crate::cosi_db::controller::common::get_connection;

use crate::cosi_db::model::address::Address;
use crate::cosi_db::model::common::{COSICollection, Generator};
use crate::cosi_db::model::person::Person;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Household {
    pub house_name: String,
    pub address: ObjectId,
    pub persons: Vec<ObjectId>,
}

#[async_trait]
impl Generator<Household> for Household {
    fn generate(size: u32) -> Vec<Household> {
        // Generates data dependent on "address" and "person" tables.
        let mut result = Vec::new();
        for i in 0..size {
            let person_col = <Person as COSICollection<Person>>::get_connection().await;
            let address_col = <Address as COSICollection<Address>>::get_connection().await;
        }

        return result;
    }
}
