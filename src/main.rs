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
use crate::cosi_db::controller::common::initialize_connections;

#[launch]
async fn rocket() -> Rocket<Build> {
    initialize_connections().await;
    let rocket_build = routes::register_route(rocket::build()).attach(Template::fairing());
    rocket_build
}
