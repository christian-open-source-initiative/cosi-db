// serde
use serde_json;

// rocket
use rocket::response::content::RawJson;

// mongo
use mongodb::{bson::doc, bson::to_document, options::FindOptions};

// cosi_db
use crate::cosi_db::controller::common::PaginateData;
use crate::cosi_db::model::common::COSICollection;
use crate::cosi_db::model::common::Generator;

use crate::{generate_generators, generate_pageable_getter};

// Address
use crate::cosi_db::model::address::{Address, AddressForm};
generate_generators! { Address }
generate_pageable_getter! { Address }

// Person
use crate::cosi_db::model::person::{Person, PersonForm};
generate_generators! { Person }
generate_pageable_getter! { Person }

// Household
use crate::cosi_db::model::household::{Household, HouseholdForm};
generate_generators! { Household }
generate_pageable_getter! { Household }
