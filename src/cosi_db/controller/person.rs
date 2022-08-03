// serde
use serde_json;

// rocket
use rocket::response::content::RawJson;

// mongo
use mongodb::{bson::doc, options::FindOptions};
use rocket::futures::TryStreamExt;

// cosi_db
use crate::cosi_db::connection::{CosiDB, MongoConnection};
use crate::cosi_db::controller::common::PaginateData;
use crate::cosi_db::generator::Generator;
use crate::cosi_db::model::person::Person;
use crate::generate_pageable_getter;

async fn get_connection() -> CosiDB {
    CosiDB::new("admin", "admin", None).await.unwrap()
}

#[get("/gen_people/<total>")]
pub async fn generate_people(total: u8) -> RawJson<String> {
    #[cfg(debug_assertions)]
    {
        let connection = get_connection().await;
        let person_data = Person::generate(total as u32);

        let person_col = connection
            .client
            .database("cosi_db")
            .collection::<Person>("person");
        person_col.drop(None).await;
        person_col.insert_many(person_data, None).await;

        let total = person_col.estimated_document_count(None).await.unwrap();
        return RawJson(format!("{{\"total\": {}}}", total));
    }
    {
        return RawJson("{}".to_string());
    }
}

generate_pageable_getter! { Person, "person" }
