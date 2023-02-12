use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, from_document, to_bson, to_document, Document};
use mongodb::Client;
use rocket::futures::TryStreamExt;

use names::Name;
use serde::{Deserialize, Serialize};

use core::convert::From;
use rocket::form::{FromForm, FromFormField};

// cosi_db
use crate::cosi_db::errors::{COSIError, COSIResult};

use crate::cosi_db::model::address::Address;
use crate::cosi_db::model::common::{COSICollection, COSIForm, Generator, OID};
use crate::cosi_db::model::person::{Person, PersonImpl};

#[derive(Clone, Debug, FromFormField, Serialize, Deserialize)]
pub enum HouseRelationStatus {
    Husband,
    Wife,
    Child,
    Guardian,
    Other,
}

#[derive(Clone, Debug, FromForm, Serialize, Deserialize)]
pub struct HouseRelation {
    pub person_a: OID,
    pub person_b: OID,
    pub relation: HouseRelationStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Household {
    pub house_name: String,
    pub address: Address,
    pub persons: Vec<Person>,
    pub relations: Vec<HouseRelation>,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct HouseholdImpl {
    pub house_name: String,
    pub address: OID,
    pub persons: Vec<OID>,
    pub relations: Vec<HouseRelation>,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct HouseholdOptional {
    pub house_name: Option<String>,
    pub address: Option<String>,
    pub persons: Option<Vec<String>>,
    pub relations: Option<Vec<HouseRelation>>,
}

impl COSIForm for HouseholdImpl {}
impl COSIForm for HouseholdOptional {}

// These do not fetch OID values. Rather, the COSIForm helpers which are async can interact with the
// database. You can consider these conversions as local conversions.
impl From<Household> for HouseholdImpl {
    fn from(h: Household) -> HouseholdImpl {
        HouseholdImpl {
            house_name: h.house_name,
            address: <OID as std::default::Default>::default(),
            persons: vec![],
            relations: vec![],
        }
    }
}

impl From<HouseholdImpl> for Household {
    fn from(h: HouseholdImpl) -> Household {
        Household {
            house_name: h.house_name,
            address: <Address as std::default::Default>::default(),
            persons: vec![],
            relations: vec![],
        }
    }
}

#[async_trait]
impl COSICollection<'_, Household, HouseholdImpl, HouseholdOptional> for Household {
    fn get_table_name() -> String {
        return "household".to_string();
    }

    async fn to_impl(client: &Client, mut orm: Vec<Household>) -> COSIResult<Vec<HouseholdImpl>> {
        // Slow, fetch results each and every one.
        let collection = Self::get_collection(client).await;
        let mut queries = vec![];
        for o in &orm {
            queries.push(collection.find_one(doc! {"house_name": o.house_name.clone()}, None));
        }
        let q_result = futures::future::join_all(queries).await;

        let address_raw = Address::get_raw_document(client).await;
        let people_raw = Person::get_raw_document(client).await;
        let mut results: Vec<HouseholdImpl> = vec![];

        for r in q_result.iter().rev() {
            // Should never error out.
            let opt = r.as_ref().unwrap();
            let orm_i = orm.pop().unwrap();

            match opt {
                Some(h) => {
                    results.push(h.clone());
                }
                None => {
                    let addr_doc = address_raw
                        .find_one(to_document(&orm_i.address)?, None)
                        .await?
                        .ok_or(COSIError::msg("Unable to fetch address table."))?;
                    let persons_doc: Vec<Document> = orm_i
                        .persons
                        .iter()
                        .map(|p| to_document(&p).unwrap())
                        .collect();
                    let people_cursor = people_raw.find(doc! {"$or": persons_doc}, None).await?;

                    let persons_results: Vec<Document> = people_cursor.try_collect().await?;
                    let persons_id: Vec<OID> = persons_results
                        .iter()
                        .map(|pd| pd.get("_id").unwrap().as_object_id().unwrap().into())
                        .collect();
                    results.push(HouseholdImpl {
                        house_name: orm_i.house_name.clone(),
                        address: addr_doc.get("_id").unwrap().as_object_id().unwrap().into(),
                        persons: persons_id,
                        relations: orm_i.relations.clone(),
                    })
                }
            }
        }

        return Ok(results);
    }

    async fn to_orm(client: &Client, imp: &Vec<HouseholdImpl>) -> COSIResult<Vec<Household>> {
        let mut result = vec![];

        let address_col = Address::get_collection(client).await;
        let person_col = Person::get_collection(client).await;
        for i in imp {
            let address = address_col
                .find_one(doc! {"_id": ObjectId::from(i.address.clone())}, None)
                .await?
                .ok_or(COSIError::msg("Unable to find provided address."))?;
            let persons_cursor = person_col
                .find(
                    doc! {"_id": {"$in": OID::vec_to_object_id(&i.persons)}},
                    None,
                )
                .await?;
            let person_impl: Vec<PersonImpl> = persons_cursor.try_collect().await?;
            let persons: Vec<Person> = person_impl.iter().map(|x| x.clone().into()).collect();

            result.push(Household {
                house_name: i.house_name.clone(),
                address: address,
                persons: persons,
                relations: i.relations.clone(),
            })
        }

        return Ok(result);
    }

    async fn process_foreign_keys<'b>(client: &'b Client, raw_doc: &'b mut Vec<Document>) {
        let h_impls: Vec<HouseholdImpl> = raw_doc
            .iter()
            .map(|x| from_document(x.clone()).unwrap())
            .collect();
        // This will fetch the foreign keys for us.
        let orms: Vec<Household> = Self::to_orm(client, &h_impls).await.unwrap();
        let it = raw_doc.iter_mut().zip(orms);
        for (rd, o) in it {
            rd.insert("address", to_bson(&o.address).unwrap());
            rd.insert("persons", to_bson(&o.persons).unwrap());
        }
    }
}

#[async_trait]
impl Generator<Household> for Household {
    async fn generate(client: &Client, size: u32) -> COSIResult<Vec<Household>> {
        // Generates data dependent on "address" and "person" tables.
        // If no values exist, this function would return a vector of zero.
        let mut result = Vec::new();

        // Random sample results and link them together.
        let person_col = Person::get_collection(client).await;
        let address_col = Address::get_collection(client).await;

        let person_agg = person_col
            .aggregate([doc! {"$sample": {"size": size}}], None)
            .await?;
        let address_agg = address_col
            .aggregate([doc! {"$sample": {"size": size}}], None)
            .await?;

        let mut result_person: Vec<Document> = person_agg.try_collect().await.unwrap();
        let mut result_address: Vec<Document> = address_agg.try_collect().await.unwrap();

        let mut generator = names::Generator::with_naming(Name::Plain);
        let mut get_name = || generator.next().unwrap();

        for _ in 0..size {
            let address = result_address.pop().unwrap();
            let person = result_person.pop().unwrap();
            result.push(Household {
                house_name: get_name(),
                address: from_document(address)?,
                persons: vec![from_document(person.clone())?],
                relations: vec![HouseRelation {
                    person_a: person.get("_id").unwrap().as_object_id().unwrap().into(),
                    person_b: person.get("_id").unwrap().as_object_id().unwrap().into(),
                    relation: HouseRelationStatus::Husband,
                }],
            });
        }

        return Ok(result);
    }
}
