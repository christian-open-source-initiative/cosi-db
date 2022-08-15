use rocket::{fs::FileServer, Build, Rocket};

use super::cosi_db::controller::api::*;
use super::cosi_db::controller::auth::*;
use super::cosi_db::controller::dashboard::{index, search};

pub fn register_route(rb: Rocket<Build>) -> Rocket<Build> {
    rb.mount("/public", FileServer::from("public")).mount(
        "/",
        routes![
            index,
            generate_person,
            get_person,
            insert_person,
            generate_address,
            get_address,
            insert_address,
            generate_household,
            get_household,
            insert_household,
            search,
            // Auth
            login,
            login_logged,
            login_submit,
            logout,
            gen_login
        ],
    )
}
