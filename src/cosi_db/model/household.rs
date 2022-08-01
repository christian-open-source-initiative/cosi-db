use serde::{Deserialize, Serialize};

use mongodb::bson::oid::ObjectId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Household {
    pub house_name: String,
    pub address: ObjectId,
    pub persons: Vec<ObjectId>,
}
