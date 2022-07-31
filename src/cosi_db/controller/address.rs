// serde
use serde_json;

// rocket
use rocket::response::content::RawJson;

// mongo
use mongodb::{bson::doc, options::FindOptions};
use rocket::futures::TryStreamExt;

// cosi_db
use crate::cosi_db::common::PaginateData;
use crate::cosi_db::connection::{CosiDB, MongoConnection};
use crate::cosi_db::generator::Generator;
use crate::cosi_db::model::address::Address;

async fn get_connection() -> CosiDB {
    CosiDB::new("admin", "admin", None).await.unwrap()
}

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

#[get("/get_address?<page>")]
pub async fn get_address(page: Option<u64>) -> RawJson<String> {
    let page = page.unwrap_or(0);

    let connection = get_connection().await;
    let address_col = connection
        .client
        .database("cosi_db")
        .collection::<Address>("address");

    // Page calculate.
    let total_address: u64 = address_col.estimated_document_count(None).await.unwrap();
    let batch_size: u32 = 100;
    let total_pages: u64 = (total_address as f64 / batch_size as f64).ceil() as u64;

    let find_options = FindOptions::builder()
        .batch_size(batch_size)
        .skip(batch_size as u64 * page)
        .build();
    let data_cursor = address_col.find(doc! {}, Some(find_options)).await.unwrap();
    let data: Vec<Address> = data_cursor.try_collect().await.unwrap();

    RawJson(
        serde_json::to_string(&PaginateData {
            page: page,
            total_pages: total_pages,
            data: data,
        })
        .unwrap(),
    )
}
