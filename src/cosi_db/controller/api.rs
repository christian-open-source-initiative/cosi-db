// serde
use serde_json;

// rocket
use rocket::response::content::RawJson;

// mongo
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};

// cosi_db
use crate::cosi_db::connection::CosiDB;
use crate::cosi_db::controller::common::{get_connection, PaginateData};
use crate::cosi_db::model::common::COSICollection;
use crate::cosi_db::model::common::Generator;

use crate::{generate_generators, generate_pageable_getter};

// Address
use crate::cosi_db::model::address::Address;
generate_generators! { Address }
generate_pageable_getter! { Address }

// Person
use crate::cosi_db::model::person::Person;
generate_generators! { Person }
generate_pageable_getter! { Person }

// Household
use crate::cosi_db::model::household::Household;
generate_generators! { Household }
generate_pageable_getter! { Household }
