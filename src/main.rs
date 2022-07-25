#[macro_use] extern crate rocket;

use rocket::{Rocket, Build};
use mongodb::{bson::doc};

mod cosi_db;
use cosi_db::connection::{MongoConnection, CosiDB};
use cosi_db::person::{Person, Sex};

#[cfg(test)] mod tests;

#[get("/")]
fn index() -> &'static str {
    "hello COSI"
}

#[get("/")]
async fn generate_data() -> String {
    #[cfg(debug_assertions)]
    {
        let connection = CosiDB::new(
            "admin",
            "admin",
            None
        ).await.unwrap();

        let person_col = connection.client.database("cosi_db").collection::<Person>("person");
        let p = Person { 
            first_name: "hello".to_string(),
            middle_name: "world".to_string(),
            last_name: "lastname".to_string(),
            nicks: Vec::new(),
            dob: None,
            age: None,
            sex: Sex::Male
        };
        person_col.drop(None).await;
        person_col.insert_one(p, None).await;
        let fetched_name = person_col.find_one(doc! {"first_name": "hello"}, None).await.unwrap().unwrap().middle_name;
        let total = person_col.count_documents(doc!{}, None).await.unwrap();
        format!("{} Count {}", fetched_name, total)
    }
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let rocket_build = rocket::build().mount("/", routes![index])
                                      .mount("/debug", routes![generate_data]);

    rocket_build
}
