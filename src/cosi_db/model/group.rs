use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, from_document, to_bson, to_document, Document};
use mongodb::Client;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

use names::Name;
use serde::{Deserialize, Serialize};

use lipsum::lipsum_words_from_seed;
use rocket::form::{FromForm, FromFormField};
use rocket::futures::TryStreamExt;
use std::default::Default;

// cosi_db
use crate::cosi_db::errors::{COSIError, COSIResult};
use crate::cosi_db::model::common::{COSICollection, COSIForm, Generator, OID};
use crate::cosi_db::model::person::Person;

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct Group {
    pub group_name: String,
    pub group_desc: String,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct GroupOptional {
    pub group_name: Option<String>,
    pub group_desc: Option<String>,
}

pub type GroupImpl = Group;
impl COSIForm for GroupImpl {}
impl COSIForm for GroupOptional {}

#[async_trait]
impl COSICollection<'_, Group, GroupImpl, GroupOptional> for Group {
    fn get_table_name() -> String {
        return "group".to_string();
    }
}

#[async_trait]
impl Generator<Group> for Group {
    async fn generate(_client: &Client, size: u32) -> COSIResult<Vec<Group>> {
        let mut result = Vec::new();
        let mut rng = thread_rng();

        // TODO: Generate optional.
        for _ in 0..size {
            let seed: u64 = rng.gen_range(0, 2_u64.pow(32));
            result.push(Group {
                group_name: lipsum_words_from_seed(2, seed),
                group_desc: lipsum_words_from_seed(3, seed),
            });
        }

        return Ok(result);
    }
}

impl Default for Group {
    fn default() -> Self {
        Group {
            group_name: "ERR".to_string(),
            group_desc: "ERR".to_string(),
        }
    }
}

// Group relations
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupRelation {
    pub person: Person,
    pub group: Group,
    pub role: String,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct GroupRelationImpl {
    pub person: OID,
    pub group: OID,
    pub role: String,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct GroupRelationOptional {
    pub person: Option<String>,
    pub group: Option<String>,
    pub role: Option<String>,
}

impl COSIForm for GroupRelationImpl {}
impl COSIForm for GroupRelationOptional {}

impl From<GroupRelation> for GroupRelationImpl {
    fn from(gr: GroupRelation) -> GroupRelationImpl {
        GroupRelationImpl {
            person: <OID as std::default::Default>::default(),
            group: <OID as std::default::Default>::default(),
            role: gr.role.clone(),
        }
    }
}

impl From<GroupRelationImpl> for GroupRelation {
    fn from(gr: GroupRelationImpl) -> GroupRelation {
        GroupRelation {
            person: <Person as std::default::Default>::default(),
            group: <Group as std::default::Default>::default(),
            role: gr.role.clone(),
        }
    }
}

#[async_trait]
impl COSICollection<'_, GroupRelation, GroupRelationImpl, GroupRelationOptional> for GroupRelation {
    fn get_table_name() -> String {
        return "grouprelation".to_string();
    }

    async fn to_impl(
        client: &Client,
        mut orm: Vec<GroupRelation>,
    ) -> COSIResult<Vec<GroupRelationImpl>> {
        let collection = Self::get_collection(client).await;

        let people_raw = Person::get_raw_document(client).await;
        let group_raw = Group::get_raw_document(client).await;
        let mut results: Vec<GroupRelationImpl> = vec![];

        // Find a valid person and group.
        for o in &orm {
            let person = people_raw
                .find_one(to_document(&o.person)?, None)
                .await?
                .ok_or(COSIError::msg("Unable to fetch Person table."))?;
            let group = group_raw
                .find_one(to_document(&o.group)?, None)
                .await?
                .ok_or(COSIError::msg("Unable to fetch Group table."))?;
            results.push(GroupRelationImpl {
                person: person.get("_id").unwrap().as_object_id().unwrap().into(),
                group: group.get("_id").unwrap().as_object_id().unwrap().into(),
                role: o.role.clone(),
            });
        }

        return Ok(results);
    }

    async fn to_orm(
        client: &Client,
        imp: &Vec<GroupRelationImpl>,
    ) -> COSIResult<Vec<GroupRelation>> {
        let mut result = vec![];
        let person_col = Person::get_collection(client).await;
        let group_col = Group::get_collection(client).await;
        for i in imp {
            let person = person_col
                .find_one(doc! {"_id": ObjectId::from(i.person.clone())}, None)
                .await?
                .ok_or(COSIError::msg("Unable to find provided person."))?;
            let group = group_col
                .find_one(doc! {"_id": ObjectId::from(i.group.clone())}, None)
                .await?
                .ok_or(COSIError::msg("Unable to find provided group."))?;

            result.push(GroupRelation {
                // TODO: Inefficient conversion.
                person: Person::to_orm(client, &vec![person]).await?[0].clone(),
                group: group,
                role: i.role.clone(),
            });
        }

        return Ok(result);
    }

    async fn process_foreign_keys<'b>(client: &'b Client, raw_doc: &'b mut Vec<Document>) {
        let impls: Vec<GroupRelationImpl> = raw_doc
            .iter()
            .map(|x| from_document(x.clone()).unwrap())
            .collect();
        // This will fetch the foreign keys for us.
        let orms: Vec<GroupRelation> = Self::to_orm(client, &impls).await.unwrap();
        let it = raw_doc.iter_mut().zip(orms);
        for (rd, o) in it {
            rd.insert("person", to_bson(&o.person).unwrap());
            rd.insert("group", to_bson(&o.group).unwrap());
        }
    }
}

#[async_trait]
impl Generator<GroupRelation> for GroupRelation {
    async fn generate(client: &Client, size: u32) -> COSIResult<Vec<GroupRelation>> {
        // Generates data dependent on "address" and "person" tables.
        // If no values exist, this function would return a vector of zero.
        let mut result = Vec::new();

        // Random sample results and link them together.
        let person_col = Person::get_collection(client).await;
        let group_col = Group::get_collection(client).await;

        let person_agg = person_col
            .aggregate([doc! {"$sample": {"size": size}}], None)
            .await?;
        let group_agg = group_col
            .aggregate([doc! {"$sample": {"size": size}}], None)
            .await?;

        let mut result_person: Vec<Document> = person_agg.try_collect().await.unwrap();
        let mut result_group: Vec<Document> = group_agg.try_collect().await.unwrap();

        let mut generator = names::Generator::with_naming(Name::Plain);
        let mut get_name = || generator.next().unwrap();

        for _ in 0..size {
            let group = result_group.pop().unwrap();
            let person = result_person.pop().unwrap();
            result.push(GroupRelation {
                person: from_document(person)?,
                group: from_document(group)?,
                role: get_name(),
            });
        }

        return Ok(result);
    }
}
