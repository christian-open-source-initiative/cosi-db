use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::options::AggregateOptions;
use mongodb::{bson::doc, bson::from_document, bson::Document, options::FindOptions};
use rocket::futures::TryStreamExt;

use names::Name;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

// cosi_db
use crate::cosi_db::connection::CosiDB;
use crate::cosi_db::controller::common::get_connection;

use crate::cosi_db::model::address::Address;
use crate::cosi_db::model::common::{COSICollection, Generator};
use crate::cosi_db::model::person::Person;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Household {
    pub house_name: String,
    pub address: Address,
    pub persons: Vec<Person>,
}

#[async_trait]
impl Generator<Household> for Household {
    async fn generate(size: u32) -> Vec<Household> {
        // Generates data dependent on "address" and "person" tables.
        // If no values exist, this function would return a vector of zero.

        let mut result = Vec::new();

        // Random sample results and link them together.
        let person_col = Person::get_collection().await;
        let address_col = Address::get_collection().await;

        let person_agg = person_col
            .aggregate([doc! {"$sample": {"size": size}}], None)
            .await
            .unwrap();
        let address_agg = address_col
            .aggregate([doc! {"$sample": {"size": size}}], None)
            .await
            .unwrap();

        let result_person: Vec<Document> = person_agg.try_collect().await.unwrap();
        let result_address: Vec<Document> = address_agg.try_collect().await.unwrap();

        let mut generator = names::Generator::with_naming(Name::Plain);
        let mut get_name = || generator.next().unwrap();

        for i in 0..size {
            result.push(Household {
                house_name: get_name(),
                address: from_document(result_address[i as usize].clone()).unwrap(),
                persons: vec![from_document(result_person[i as usize].clone()).unwrap()],
            });
        }

        return result;
    }
}
