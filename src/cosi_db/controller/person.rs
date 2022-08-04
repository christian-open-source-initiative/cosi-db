// serde
use serde_json;

// rocket
use rocket::response::content::RawJson;

// mongo
use mongodb::{bson::doc, options::FindOptions};
use rocket::futures::TryStreamExt;

// cosi_db
use crate::cosi_db::connection::{CosiDB, MongoConnection};
use crate::cosi_db::controller::common::{get_connection, PaginateData};
use crate::cosi_db::generator::Generator;
use crate::cosi_db::model::person::Person;
use crate::{generate_generators, generate_pageable_getter};

generate_generators! { Person, "person" }
generate_pageable_getter! { Person, "person" }
