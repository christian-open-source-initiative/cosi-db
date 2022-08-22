use rocket::{fs::FileServer, Build, Rocket};

use super::cosi_db::controller::api::*;
use super::cosi_db::controller::auth::*;
use super::cosi_db::controller::dashboard::*;

pub fn register_route(rb: Rocket<Build>) -> Rocket<Build> {
    rb.mount("/public", FileServer::from("public")).mount(
        "/",
        routes![
            // Dashboard
            index,
            index_redirect,
            // Person
            gen_person,
            get_person,
            insert_person,
            // Address
            gen_address,
            get_address,
            insert_address,
            // Household
            gen_household,
            get_household,
            insert_household,
            // Event
            gen_event,
            get_event,
            insert_event,
            // Group
            gen_group,
            get_group,
            insert_group,
            // Group Relation
            gen_grouprelation,
            get_grouprelation,
            insert_grouprelation,
            // Search
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
