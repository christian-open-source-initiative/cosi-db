#[macro_use]
extern crate rocket;

// Rocket
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

pub use ::paste;
pub use ::with_builtin_macros;

// COSI
pub mod cosi_db;
pub mod routes;
use crate::cosi_db::connection::COSIMongo;
use rocket_db_pools::Database;

#[launch]
async fn rocket() -> Rocket<Build> {
    let rocket_build = routes::register_route(rocket::build())
        .attach(Template::fairing())
        .attach(COSIMongo::init());
    rocket_build
}
