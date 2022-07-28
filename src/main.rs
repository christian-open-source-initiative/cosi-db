#[macro_use]
extern crate rocket;

// Rocket
use rocket::{Build, Rocket};
use rocket_dyn_templates::{Template};

// COSI
pub mod cosi_db;
pub mod routes;



#[launch]
async fn rocket() -> Rocket<Build> {
    let rocket_build = routes::register_route(rocket::build())
                            .attach(Template::fairing());
    rocket_build
}
