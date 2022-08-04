// serde
use serde_json;

// rocket
use rocket::response::content::RawJson;

// mongo
use mongodb::{bson::doc, options::FindOptions};
use rocket::futures::TryStreamExt;

// cosi_db
use crate::cosi_db::connection::CosiDB;
use crate::cosi_db::controller::common::{get_connection, PaginateData};
use crate::cosi_db::model::common::Generator;

use crate::{generate_generators, generate_pageable_getter};

// Address
use crate::cosi_db::model::address::Address;
generate_generators! { Address, "address" }
generate_pageable_getter! { Address, "address" }

// Person
use crate::cosi_db::model::person::Person;
generate_generators! { Person, "person" }
generate_pageable_getter! { Person, "person" }

// Household
use crate::cosi_db::model::household::Household;
generate_generators! { Household, "household" }
generate_pageable_getter! { Household, "household" }
