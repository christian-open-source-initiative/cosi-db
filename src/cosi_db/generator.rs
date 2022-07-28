extern crate rand;

use chrono::NaiveDate;
use names::Name;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

// COSI
use super::model::person::{Person, Sex};

pub trait Generator<T> {
    fn generate(size: u32) -> Vec<T>;
}

impl Generator<Sex> for Sex {
    fn generate(size: u32) -> Vec<Sex> {
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

impl Generator<Person> for Person {
    fn generate(size: u32) -> Vec<Person> {
        let mut result = Vec::new();
        let mut generator = names::Generator::with_naming(Name::Plain);
        let mut get_name = || generator.next().unwrap();
        let mut rng = thread_rng();
        let sexes = Sex::generate(size);

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
