//std
use std::collections::HashMap;
use std::str::FromStr;

// serde
use serde_json;

// rocket
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::content::RawJson;
use rocket::response::status::Custom;
use rocket_db_pools::Connection;

// mongo
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, from_document, to_bson, Bson, Document};
use mongodb::options::FindOptions;
use mongodb::Client;

// cosi_db
use crate::cosi_db::connection::COSIMongo;
use crate::cosi_db::controller::common::PaginateData;
use crate::cosi_db::errors::COSIResult;
use crate::cosi_db::model::auth::User;
use crate::cosi_db::model::common::{COSICollection, Generator};

use crate::{
    generate_dropper, generate_generators, generate_pageable_find, generate_pageable_getter,
    generate_pageable_inserter, generate_pageable_update,
};

// Address
use crate::cosi_db::model::address::{Address, AddressImpl, AddressOptional};
generate_generators! { Address }
generate_pageable_find! { Address }
generate_pageable_inserter! { Address }
generate_dropper! { Address }
generate_pageable_update! { Address }

// Person
use crate::cosi_db::model::person::{Person, PersonImpl, PersonOptional};
generate_generators! { Person }
generate_pageable_find! { Person }
generate_pageable_getter! { Person }
generate_pageable_inserter! { Person }
generate_dropper! { Person }
generate_pageable_update! { Person }

// Household
use crate::cosi_db::model::household::{Household, HouseholdImpl, HouseholdOptional};
generate_generators! { Household }
generate_pageable_find! { Household }
generate_pageable_inserter! { Household }
generate_dropper! { Household }

// Event
use crate::cosi_db::model::event::{Event, EventImpl, EventOptional};
generate_generators! { Event }
generate_pageable_find! { Event }
generate_pageable_inserter! { Event }
generate_dropper! { Event }
generate_pageable_update! { Event }

// Event Registration
use crate::cosi_db::model::event::{
    EventRegistration, EventRegistrationImpl, EventRegistrationOptional,
};
generate_generators! { EventRegistration }
generate_pageable_find! { EventRegistration }
generate_pageable_inserter! { EventRegistration }
generate_dropper! { EventRegistration }

// Group
use crate::cosi_db::model::group::{Group, GroupImpl, GroupOptional};
generate_generators! { Group }
generate_pageable_find! { Group }
generate_pageable_inserter! { Group }
generate_dropper! { Group }
generate_pageable_update! { Group }

// Group Relation
use crate::cosi_db::model::group::{GroupRelation, GroupRelationImpl, GroupRelationOptional};
generate_generators! { GroupRelation }
generate_pageable_find! { GroupRelation }
generate_pageable_inserter! { GroupRelation }
generate_dropper! { GroupRelation }
