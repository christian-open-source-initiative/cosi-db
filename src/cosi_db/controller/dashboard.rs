use rocket::response::content::{RawHtml, RawJson};
use rocket_dyn_templates::{context, Template};

// mongo
use mongodb::bson::doc;

use crate::cosi_db::model::address::{Address, AddressForm};
use crate::cosi_db::model::common::COSICollection;
use crate::cosi_db::model::household::Household;
use crate::cosi_db::model::person::Person;

#[get("/")]
pub fn index() -> RawHtml<Template> {
    RawHtml(Template::render("dashboard", context! {}))
}

#[get("/search?<query>")]
pub async fn search(query: &str) -> RawJson<String> {
    // TODO add tables parameter.
    let address_data: Vec<Address> = Address::find_data(
        Some(doc! {"line_one": {"$regex": format!("(?i).*{}.*", query.to_lowercase())}}),
        None,
    )
    .await
    .unwrap();

    RawJson(format!(
        "[{}]",
        serde_json::to_string(&address_data).unwrap()
    ))
}
