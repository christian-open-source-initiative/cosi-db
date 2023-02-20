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
            find_person,
            get_person,
            insert_person,
            drop_person,
            update_person,
            // Address
            gen_address,
            find_address,
            insert_address,
            drop_address,
            update_address,
            // Household
            gen_household,
            find_household,
            insert_household,
            drop_household,
            // Event
            gen_event,
            find_event,
            insert_event,
            drop_event,
            update_event,
            // Event Registration
            gen_eventregistration,
            find_eventregistration,
            insert_eventregistration,
            drop_eventregistration,
            // Group
            gen_group,
            find_group,
            insert_group,
            drop_group,
            update_group,
            // Group Relation
            gen_grouprelation,
            find_grouprelation,
            insert_grouprelation,
            drop_grouprelation,
            // Search
            search,
            expanded_search,
            expanded_search_redirect,
            // Auth
            login,
            login_logged,
            login_submit,
            logout,
            gen_login
        ],
    )
}
