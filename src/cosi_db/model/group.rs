use async_trait::async_trait;
use mongodb::Client;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use lipsum::lipsum_words_from_seed;
use rocket::form::{FromForm, FromFormField};

// cosi_db
use crate::cosi_db::errors::{COSIError, COSIResult};
use crate::cosi_db::model::common::{COSICollection, COSIForm, Generator, OID};

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct Group {
    pub group_name: String,
    pub group_desc: String,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct GroupOptional {
    pub group_name: Option<String>,
    pub group_desc: Option<String>,
}

pub type GroupImpl = Group;
impl COSIForm for GroupImpl {}
impl COSIForm for GroupOptional {}

#[async_trait]
impl COSICollection<'_, Group, GroupImpl, GroupOptional> for Group {
    fn get_table_name() -> String {
        return "group".to_string();
    }
}

#[async_trait]
impl Generator<Group> for Group {
    async fn generate(_client: &Client, size: u32) -> COSIResult<Vec<Group>> {
        let mut result = Vec::new();
        let mut rng = thread_rng();

        // TODO: Generate optional.
        for _ in 0..size {
            let seed: u64 = rng.gen_range(0, 2_u64.pow(32));
            result.push(Group {
                group_name: lipsum_words_from_seed(2, seed),
                group_desc: lipsum_words_from_seed(3, seed),
            });
        }

        return Ok(result);
    }
}

// Group relations
#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct GroupRelation {
    pub person_id: OID,
    pub group_id: OID,
    pub role: String,
}
