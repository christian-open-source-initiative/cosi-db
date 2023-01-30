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
            drop_person,
            person,
            person_redirect,
            // Address
            gen_address,
            get_address,
            insert_address,
            drop_address,
            // Household
            gen_household,
            get_household,
            insert_household,
            drop_household,
            // Event
            gen_event,
            get_event,
            insert_event,
            drop_event,
            // Event Registration
            gen_eventregistration,
            get_eventregistration,
            insert_eventregistration,
            drop_eventregistration,
            // Group
            gen_group,
            get_group,
            insert_group,
            drop_group,
            // Group Relation
            gen_grouprelation,
            get_grouprelation,
            insert_grouprelation,
            drop_grouprelation,
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
