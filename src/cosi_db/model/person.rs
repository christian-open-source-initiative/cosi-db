use async_trait::async_trait;
use chrono::NaiveDate;
use mongodb::bson::doc;
use mongodb::Client;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use names::Name;
use rocket::form::{FromForm, FromFormField};

// cosi_db
use super::common::{COSICollection, COSIForm, Generator};
use crate::cosi_db::errors::{COSIError, COSIResult};

#[derive(Copy, Clone, Debug, FromFormField, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct PersonImpl {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub nicks: Vec<String>, // Vectors default to empty array.
    pub dob: Option<String>,
    pub age: Option<u8>,
    pub sex: Sex,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct PersonOptional {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub nicks: Option<Vec<String>>,
    pub dob: Option<Option<String>>,
    pub age: Option<Option<u8>>,
    pub sex: Option<Sex>,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            first_name: "".to_string(),
            middle_name: "".to_string(),
            last_name: "".to_string(),
            nicks: vec![],
            dob: None,
            age: None,
            sex: Sex::Undefined,
        }
    }
}

impl From<Person> for PersonImpl {
    fn from(p: Person) -> PersonImpl {
        PersonImpl {
            first_name: p.first_name,
            middle_name: p.middle_name,
            last_name: p.last_name,
            nicks: p.nicks,
            dob: p.dob.map(|x| x.to_string()),
            age: p.age,
            sex: p.sex,
        }
    }
}

impl From<PersonImpl> for Person {
    fn from(p: PersonImpl) -> Person {
        Person {
            first_name: p.first_name,
            middle_name: p.middle_name,
            last_name: p.last_name,
            nicks: p.nicks,
            dob: p
                .dob
                .map(|x| NaiveDate::parse_from_str(&x, "%Y-%m-%d").unwrap()),
            age: p.age,
            sex: p.sex,
        }
    }
}

impl PersonOptional {
    fn _sanitize(form: &PersonOptional) -> COSIResult<()> {
        let check = |b: bool, err_msg: Vec<&str>| {
            if !b {
                Err(COSIError::msg(err_msg.join(" ")))
            } else {
                Ok(true)
            }
        };
        if let Some(Some(dob)) = &form.dob {
            let splits: Vec<&str> = dob.split("-").collect();
            let err_msg = "Date should be <year>-<month>-<day>.";
            check(splits.len() == 3, vec![err_msg])?;

            let year = splits[0].parse::<u16>();
            let month = splits[1].parse::<u8>();
            let day = splits[2].parse::<u8>();
            check(year.is_ok(), vec![err_msg, "Invalid year number."])?;
            check(month.is_ok(), vec![err_msg, "Invalid month number."])?;
            check(day.is_ok(), vec![err_msg, "Invalid day number."])?;

            // Force year to be within the 1800+
            check(year.unwrap() > 1800, vec!["Year must be greater than 1800"])?;

            // Rest of the errors.
            NaiveDate::parse_from_str(&dob, "%Y-%m-%d")?;
        }

        return Ok(());
    }
}

impl COSIForm for PersonImpl {
    fn sanitize_insert(&self) -> COSIResult<mongodb::bson::Document>
    where
        Self: Serialize,
    {
        // Add some common user errors.
        // Reuse the same santize definition.
        // TODO: into syntax is possible here.
        PersonOptional::_sanitize(&PersonOptional {
            first_name: Some(self.first_name.clone()),
            middle_name: Some(self.middle_name.clone()),
            last_name: Some(self.last_name.clone()),
            nicks: Some(self.nicks.clone()),
            dob: Some(self.dob.clone()),
            age: Some(self.age),
            sex: Some(self.sex),
        })?;
        return self.convert_to_document(true);
    }
}

impl COSIForm for PersonOptional {
    fn sanitize_insert(&self) -> COSIResult<mongodb::bson::Document>
    where
        Self: Serialize,
    {
        // Add some common user errors.
        PersonOptional::_sanitize(self)?;
        return self.convert_to_document(false);
    }
}

#[async_trait]
impl COSICollection<'_, Person, PersonImpl, PersonOptional> for Person {
    fn get_table_name() -> String {
        return "person".to_string();
    }
}

#[async_trait]
impl Generator<Sex> for Sex {
    async fn generate(_client: &Client, size: u32) -> COSIResult<Vec<Sex>> {
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

        return Ok(result);
    }
}

#[async_trait]
impl Generator<Person> for Person {
    async fn generate(client: &Client, size: u32) -> COSIResult<Vec<Person>> {
        let sexes = Sex::generate(client, size).await?;
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
                nicks: if age < 20 {
                    vec![]
                } else {
                    vec![get_name(), get_name()]
                },
                dob: Some(gen_date(age, &mut rng)),
                age: Some(age),
                sex: sexes[i as usize],
            });
        }

        return Ok(result);
    }
}
