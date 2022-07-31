use rocket::{fs::FileServer, Build, Rocket};

use super::cosi_db::controller::address::{generate_address, get_address};
use super::cosi_db::controller::dashboard::index;
use super::cosi_db::controller::person::{generate_people, get_people};

pub fn register_route(rb: Rocket<Build>) -> Rocket<Build> {
    rb.mount("/public", FileServer::from("public")).mount(
        "/",
        routes![
            index,
            generate_people,
            get_people,
            generate_address,
            get_address
        ],
    )
}
