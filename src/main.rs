#[macro_use]
extern crate rocket;

// Rocket
use rocket::response::content::{RawHtml, RawJson};
use rocket::{fs::FileServer, Build, Rocket};
use rocket_dyn_templates::{context, Template};

// Mongo
use mongodb::{bson::doc, options::FindOptions};
use rocket::futures::TryStreamExt;

// serde
use serde_json;

// COSI
mod cosi_db;
use cosi_db::common::PaginateData;
use cosi_db::connection::{CosiDB, MongoConnection};
use cosi_db::person::{Person};
use cosi_db::generator::{Generator};

async fn get_connection() -> CosiDB {
    CosiDB::new("admin", "admin", None).await.unwrap()
}

#[get("/")]
fn index() -> RawHtml<Template> {
    RawHtml(Template::render("dashboard", context! {}))
}

#[get("/gen_people/<total>")]
async fn generate_people(total: u8) -> RawJson<String> {
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

// Hardcoded return max of 100.
#[get("/get_people?<page>")]
async fn get_people(page: Option<u64>) -> RawJson<String> {
    let page = page.unwrap_or(0);

    let connection = get_connection().await;
    let person_col = connection
        .client
        .database("cosi_db")
        .collection::<Person>("person");

    // Page calculate.
    let total_people: u64 = person_col.estimated_document_count(None).await.unwrap();
    let batch_size: u32 = 100;
    let total_pages: u64 = (total_people as f64 / batch_size as f64).ceil() as u64;

    let find_options = FindOptions::builder()
        .batch_size(batch_size)
        .skip(batch_size as u64 * page)
        .build();
    let data_cursor = person_col.find(doc! {}, Some(find_options)).await.unwrap();
    let data: Vec<Person> = data_cursor.try_collect().await.unwrap();

    RawJson(
        serde_json::to_string(&PaginateData {
            page: page,
            total_pages: total_pages,
            data: data,
        })
        .unwrap(),
    )
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let rocket_build = rocket::build()
        .mount("/public", FileServer::from("public"))
        .mount("/", routes![index, generate_people, get_people])
        .attach(Template::fairing());

    rocket_build
}
