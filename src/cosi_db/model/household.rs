use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::options::AggregateOptions;
use mongodb::{bson::doc, bson::from_document, bson::Document, options::FindOptions};
use rocket::futures::TryStreamExt;

use names::Name;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use core::convert::From;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HouseholdImpl {
    pub house_name: String,
    pub address: ObjectId,
    pub persons: Vec<ObjectId>,
}

impl From<Household> for HouseholdImpl {
    fn from(h: Household) -> HouseholdImpl {
        HouseholdImpl {
            house_name: h.house_name,
            address: ObjectId::default(),
            persons: vec![],
        }
    }
}

impl From<HouseholdImpl> for Household {
    fn from(h: HouseholdImpl) -> Household {
        Household {
            house_name: h.house_name,
            address: Address::default(),
            persons: vec![],
        }
    }
}

#[async_trait]
impl COSICollection<'_, Household, HouseholdImpl> for Household {
    async fn get_collection() -> mongodb::Collection<HouseholdImpl> {
        get_connection()
            .await
            .collection::<HouseholdImpl>("household")
    }

    async fn to_impl(orm: Vec<Household>) -> Vec<HouseholdImpl> {
        let mut result: Vec<HouseholdImpl> = vec![];

        // Slow, fetch results each and every one.
        let collection = Self::get_collection().await;
        let mut queries = vec![];
        for o in orm {
            queries.push(collection.find_one(doc! {"house_name": o.house_name}, None));
        }
        return futures::future::join_all(queries)
            .await
            .iter()
            .map(|v| v.clone().unwrap().unwrap())
            .collect();
    }

    async fn to_orm(imp: Vec<HouseholdImpl>) -> Vec<Household> {
        let mut result = vec![];

        let address_col = Address::get_collection().await;
        let person_col = Person::get_collection().await;
        for i in imp {
            let address = address_col
                .find_one(doc! {"_id": i.address}, None)
                .await
                .unwrap()
                .unwrap();
            let persons_cursor = person_col
                .find(doc! {"_id": {"$in": i.persons}}, None)
                .await
                .unwrap();
            let persons = persons_cursor.try_collect().await.unwrap();

            result.push(Household {
                house_name: i.house_name,
                address: address,
                persons: persons,
            })
        }

        return result;
    }
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

        let mut result_person: Vec<Document> = person_agg.try_collect().await.unwrap();
        let mut result_address: Vec<Document> = address_agg.try_collect().await.unwrap();

        let mut generator = names::Generator::with_naming(Name::Plain);
        let mut get_name = || generator.next().unwrap();

        for i in 0..size {
            let address = result_address.pop().unwrap();
            let person = result_person.pop().unwrap();
            result.push(Household {
                house_name: get_name(),
                address: from_document(address).unwrap(),
                persons: vec![from_document(person).unwrap()],
            });
        }

        return result;
    }
}
