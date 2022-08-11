// serde
use serde_json;

// rocket
use rocket::http::Status;
use rocket::response::content::RawJson;
use rocket::response::status::Custom;

// mongo
use mongodb::{bson::doc, bson::from_document, bson::Bson, options::FindOptions};

// cosi_db
use crate::cosi_db::controller::common::PaginateData;
use crate::cosi_db::model::common::COSICollection;
use crate::cosi_db::model::common::Generator;

use crate::{generate_generators, generate_pageable_getter, generate_pageable_inserter};

// Address
use crate::cosi_db::model::address::{Address, AddressForm};
generate_generators! { Address }
generate_pageable_getter! { Address }
generate_pageable_inserter! { Address }

// Person
use crate::cosi_db::model::person::{Person, PersonForm};
generate_generators! { Person }
generate_pageable_getter! { Person }
generate_pageable_inserter! { Person }

// Household
use crate::cosi_db::model::household::{Household, HouseholdForm};
generate_generators! { Household }
generate_pageable_getter! { Household }
generate_pageable_inserter! { Household }
