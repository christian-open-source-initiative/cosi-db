use rocket::response::content::{RawHtml, RawJson};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

// mongo
use mongodb::bson::doc;
use mongodb::Client;

// rocket
use rocket::response::{Flash, Redirect};

// cosi_db
use crate::cosi_db::connection::COSIMongo;
use crate::cosi_db::model::address::Address;
use crate::cosi_db::model::auth::User;
use crate::cosi_db::model::common::COSICollection;
use crate::cosi_db::model::household::Household;
use crate::cosi_db::model::person::Person;

#[get("/", rank = 2)]
pub fn index(_user: User) -> RawHtml<Template> {
    RawHtml(Template::render("dashboard", context! {}))
}

#[get("/", rank = 3)]
pub fn index_redirect() -> Flash<Redirect> {
    Flash::success(Redirect::to("/login"), "User needs to be logged in.")
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SearchTable<T> {
    data: T,
    entry_match: String,
}

#[get("/person", rank = 2)]
pub async fn person(_user: User) -> RawHtml<Template> {
    RawHtml(Template::render("person", context! {}))
}

#[get("/person", rank = 3)]
pub fn person_redirect() -> Flash<Redirect> {
    Flash::success(Redirect::to("/person"), "User needs to be logged in.")
}

#[get("/search?<query>")]
pub async fn search(_user: User, connect: Connection<COSIMongo>, query: &str) -> RawJson<String> {
    let client: &Client = &*connect;

    // TODO add tables parameter.
    let rstring = format!("(?i).*{}.*", query.to_lowercase());
    let a_entry = vec!["line_one", "line_two", "line_three", "city"];
    let mut address_data: Vec<SearchTable<Address>> = Vec::new();

    // Naively search each entry for potential related values.
    for entry in a_entry {
        let av = Address::find_data(
            client,
            Some(doc! {entry: {"$regex": rstring.clone()}}),
            None,
        )
        .await
        .unwrap();
        let mut address_result: Vec<SearchTable<Address>> = av
            .iter()
            .map(|x| SearchTable {
                data: x.clone(),
                entry_match: entry.to_string(),
            })
            .collect();
        address_data.append(&mut address_result);
    }

    let h_entry = vec!["house_name"];
    let mut household_data: Vec<SearchTable<Household>> = Vec::new();

    for entry in h_entry {
        let av = Household::find_data(
            client,
            Some(doc! {entry: {"$regex": rstring.clone()}}),
            None,
        )
        .await
        .unwrap();
        let mut household_result: Vec<SearchTable<Household>> = av
            .iter()
            .map(|x| SearchTable {
                data: x.clone(),
                entry_match: entry.to_string(),
            })
            .collect();
        household_data.append(&mut household_result);
    }

    let p_entry = vec!["first_name", "middle_name", "last_name"];
    let mut person_data: Vec<SearchTable<Person>> = Vec::new();
    for entry in p_entry {
        let av = Person::find_data(
            client,
            Some(doc! {entry: {"$regex": rstring.clone()}}),
            None,
        )
        .await
        .unwrap();
        let mut person_result: Vec<SearchTable<Person>> = av
            .iter()
            .map(|x| SearchTable {
                data: x.clone(),
                entry_match: entry.to_string(),
            })
            .collect();
        person_data.append(&mut person_result);
    }

    RawJson(format!(
        "{{ \"Address\": {}, \"Household\": {}, \"Person\": {}}}",
        serde_json::to_string(&address_data).unwrap(),
        serde_json::to_string(&household_data).unwrap(),
        serde_json::to_string(&person_data).unwrap(),
    ))
}
