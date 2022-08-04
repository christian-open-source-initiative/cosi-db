use async_trait::async_trait;
use chrono::NaiveDate;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use names::Name;

// cosi_db
use super::common::{COSICollection, Generator};
use crate::cosi_db::controller::common::get_connection;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Sex {
    Male,
    Female,
    Undefined,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub nicks: Vec<String>,
    pub dob: Option<NaiveDate>,
    pub age: Option<u8>,
    pub sex: Sex,
}

#[async_trait]
impl COSICollection<'_, Person, Person> for Person {
    async fn get_collection() -> mongodb::Collection<Person> {
        get_connection().await.collection::<Person>("person")
    }
}

#[async_trait]
impl Generator<Sex> for Sex {
    async fn generate(size: u32) -> Vec<Sex> {
        let mut rng = thread_rng();
        let mut result = Vec::new();

        for _ in 0..size {
            let val = rng.gen_range(0, 3);
            let pick = match val {
                0 => Sex::Male,
                1 => Sex::Female,
                _ => Sex::Undefined,
            };
            result.push(pick);
        }

        return result;
    }
}

#[async_trait]
impl Generator<Person> for Person {
    async fn generate(size: u32) -> Vec<Person> {
        let sexes = Sex::generate(size).await;
        let mut result = Vec::new();
        let mut generator = names::Generator::with_naming(Name::Plain);
        let mut get_name = || generator.next().unwrap();
        let mut rng = thread_rng();

        let gen_date = |age: u8, rng: &mut ThreadRng| {
            NaiveDate::from_ymd(
                2022 - age as i32,
                rng.gen_range(1, 12),
                rng.gen_range(1, 28),
            )
        };

        for i in 0..size {
            let age: u8 = rng.gen_range(0, 119);
            result.push(Person {
                first_name: get_name(),
                middle_name: get_name(),
                last_name: get_name(),
                nicks: vec![],
                dob: Some(gen_date(age, &mut rng)),
                age: Some(age),
                sex: sexes[i as usize],
            });
        }

        return result;
    }
}
