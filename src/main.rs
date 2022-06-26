#[macro_use] extern crate rocket;

use rocket::{Rocket, Build};

mod cosi_db;
use cosi_db::connection::{MongoConnection, CosiDB};

#[cfg(test)] mod tests;

#[get("/")]
fn index() -> &'static str {
    "hi"
}

#[get("/")]
async fn generate_data() -> &'static str {
    #[cfg(debug_assertions)]
    {
        let result = CosiDB::new(
            "mongodb://admin:admin@localhost:27017"
        ).await.unwrap();
        "generating string complete"
    }
}

#[launch]
async fn rocket() -> Rocket<Build> {
    let rocket_build = rocket::build().mount("/", routes![index])
                                      .mount("/debug", routes![generate_data]);

    rocket_build
}
