use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use mongodb::bson::doc;
use mongodb::Client;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::cmp;

use rocket::form::{FromForm, FromFormField};

// cosi_db
use super::common::{COSICollection, COSIForm, Generator};
use crate::cosi_db::errors::{COSIError, COSIResult};

#[derive(Copy, Clone, Debug, FromFormField, Deserialize, Serialize)]
pub enum Reoccurring {
    Days,
    Weeks,
    Months,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    meeting_days: Vec<u8>,
    start_datetime: NaiveDateTime,
    end_datetime: Option<NaiveDateTime>,
    freq: u8,
    reoccuring: Option<Reoccurring>,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct EventImpl {
    meeting_days: Vec<u8>,
    start_datetime: String,
    end_datetime: Option<String>,
    freq: u8,
    reoccuring: Option<Reoccurring>,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct EventOptional {
    meeting_days: Option<Vec<u8>>,
    start_datetime: Option<String>,
    end_datetime: Option<Option<String>>,
    freq: Option<u8>,
    reoccuring: Option<Reoccurring>,
}

impl From<Event> for EventImpl {
    fn from(e: Event) -> EventImpl {
        EventImpl {
            meeting_days: e.meeting_days,
            start_datetime: e.start_datetime.to_string(),
            end_datetime: e.end_datetime.map(|x| x.to_string()),
            freq: e.freq,
            reoccuring: e.reoccuring,
        }
    }
}

impl From<EventImpl> for Event {
    fn from(e: EventImpl) -> Event {
        let parse_date_str =
            |v: &str| NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S").unwrap();
        Event {
            meeting_days: e.meeting_days,
            start_datetime: parse_date_str(&e.start_datetime),
            end_datetime: e.end_datetime.map(|x| parse_date_str(&x)),
            freq: e.freq,
            reoccuring: e.reoccuring,
        }
    }
}

impl COSIForm for EventImpl {}

impl COSIForm for EventOptional {}

#[async_trait]
impl COSICollection<'_, Event, EventImpl, EventOptional> for Event {
    fn get_table_name() -> String {
        return "event".to_string();
    }
}

#[async_trait]
impl Generator<Event> for Event {
    async fn generate(_client: &Client, size: u32) -> COSIResult<Vec<Event>> {
        let mut result = Vec::new();
        let mut rng = thread_rng();

        // Generates only multi-day events.
        let gen_date = |offset_day: u8, rng: &mut ThreadRng| {
            NaiveDate::from_ymd(
                2022,
                rng.gen_range(1, 12),
                cmp::max(rng.gen_range(1, 28) - offset_day as u32, 1),
            )
            .and_hms(7, 7, 7)
        };

        for _ in 0..size {
            result.push(Event {
                meeting_days: vec![0, 1],
                start_datetime: gen_date(1, &mut rng),
                end_datetime: Some(gen_date(0, &mut rng)),
                freq: 0,
                reoccuring: None,
            });
        }
        return Ok(result);
    }
}