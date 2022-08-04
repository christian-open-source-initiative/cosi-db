// serde
use serde_json;

// rocket
use rocket::response::content::RawJson;

// mongo
use mongodb::{bson::doc, options::FindOptions};
use rocket::futures::TryStreamExt;

// cosi_db
use crate::cosi_db::connection::{CosiDB, MongoConnection};
use crate::cosi_db::controller::common::{get_connection, PaginateData};
use crate::cosi_db::generator::Generator;
use crate::cosi_db::model::address::Address;
use crate::generate_pageable_getter;

#[get("/gen_address/<total>")]
pub async fn generate_address(total: u8) -> RawJson<String> {
    #[cfg(debug_assertions)]
    {
        let connection = get_connection().await;
        let address_data = Address::generate(total as u32);
        let address_col = connection
            .client
            .database("cosi_db")
            .collection::<Address>("address");
        address_col.drop(None).await;
        address_col.insert_many(address_data, None).await;

        let total = address_col.estimated_document_count(None).await.unwrap();
        return RawJson(format!("{{\"total\": {}}}", total));
    }
    {
        return RawJson("{}".to_string());
    }
}

generate_pageable_getter! { Address, "address" }
