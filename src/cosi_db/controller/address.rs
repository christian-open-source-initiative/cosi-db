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
use crate::cosi_db::model::address::Address;
use crate::cosi_db::model::common::Generator;

use crate::{generate_generators, generate_pageable_getter};

generate_generators! { Address, "address" }
generate_pageable_getter! { Address, "address" }
