// serde
use serde_json;

// rocket
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::content::RawJson;
use rocket::response::status::Custom;
use rocket_db_pools::Connection;

// mongo
use mongodb::bson::{doc, from_document, Bson, Document};
use mongodb::options::FindOptions;
use mongodb::Client;

// cosi_db
use crate::cosi_db::connection::COSIMongo;
use crate::cosi_db::controller::common::PaginateData;
use crate::cosi_db::model::common::COSICollection;
use crate::cosi_db::model::common::Generator;

use crate::{generate_generators, generate_pageable_getter, generate_pageable_inserter};

// Address
use crate::cosi_db::model::address::{Address, AddressImpl, AddressOptional};
generate_generators! { Address }
generate_pageable_getter! { Address }
generate_pageable_inserter! { Address }

// Person
use crate::cosi_db::model::person::{Person, PersonImpl, PersonOptional};
generate_generators! { Person }
generate_pageable_getter! { Person }
generate_pageable_inserter! { Person }

// Household
use crate::cosi_db::model::household::{Household, HouseholdImpl, HouseholdOptional};
generate_generators! { Household }
generate_pageable_getter! { Household }
generate_pageable_inserter! { Household }
