use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, from_document, to_bson, to_document, Document};
use mongodb::Client;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::cmp;

use rocket::form::{FromForm, FromFormField};
use rocket::futures::TryStreamExt;

// cosi_db
use crate::cosi_db::errors::{COSIError, COSIResult};
use crate::cosi_db::model::address::{Address, AddressImpl};
use crate::cosi_db::model::common::{COSICollection, COSIForm, Generator, OID};
use crate::cosi_db::model::group::{Group, GroupImpl};
use crate::cosi_db::model::household::{Household, HouseholdImpl};
use crate::cosi_db::model::person::{Person, PersonImpl};

#[derive(Copy, Clone, Debug, FromFormField, Deserialize, Serialize)]
pub enum Reoccurring {
    Days,
    Weeks,
    Months,
}

#[derive(Copy, Clone, Debug, FromFormField, Deserialize, Serialize)]
pub enum Days {
    M,
    Tu,
    W,
    Th,
    F,
    Sa,
    Su,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub meeting_days: Vec<Days>,
    pub start_datetime: NaiveDateTime,
    pub end_datetime: Option<NaiveDateTime>,
    pub freq: u8,
    pub reoccuring: Option<Reoccurring>,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct EventImpl {
    pub meeting_days: Vec<Days>,
    pub start_datetime: String,
    pub end_datetime: Option<String>,
    pub freq: u8,
    pub reoccuring: Option<Reoccurring>,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct EventOptional {
    pub meeting_days: Option<Vec<Days>>,
    pub start_datetime: Option<String>,
    pub end_datetime: Option<Option<String>>,
    pub freq: Option<u8>,
    pub reoccuring: Option<Reoccurring>,
}

impl Default for Event {
    fn default() -> Self {
        Event {
            meeting_days: vec![],
            start_datetime: NaiveDate::from_ymd(2020, 6, 7).and_hms(7, 7, 7),
            end_datetime: None,
            freq: 0,
            reoccuring: None,
        }
    }
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
        let create_date = |month: u8, day: u8| {
            NaiveDate::from_ymd(2022, cmp::min(month.into(), 12), cmp::min(day.into(), 28))
                .and_hms(7, 7, 7)
        };

        for _ in 0..size {
            let start_day = rng.gen_range(2, 28);
            let start_month = rng.gen_range(1, 12);
            result.push(Event {
                meeting_days: vec![Days::M, Days::W],
                start_datetime: create_date(start_month, start_day),
                end_datetime: Some(create_date(start_month, start_day + rng.gen_range(2, 17))),
                freq: 0,
                reoccuring: None,
            });
        }
        return Ok(result);
    }
}

#[derive(Copy, Clone, Debug, FromFormField, Deserialize, Serialize)]
pub enum EventKeyType {
    Group,
    Household,
    Person,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventRegistration {
    pub event: Event,
    pub timestamp: NaiveDateTime,
    pub person: Option<Person>,
    pub group: Option<Group>,
    pub household: Option<Household>,
    pub key_type: EventKeyType,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct EventRegistrationImpl {
    pub event: OID,
    pub timestamp: String,
    pub person: Option<OID>,
    pub group: Option<OID>,
    pub household: Option<OID>,
    pub key_type: EventKeyType,
}

#[derive(Clone, Debug, Deserialize, FromForm, Serialize)]
pub struct EventRegistrationOptional {
    pub event: Option<Option<OID>>,
    pub timestamp: Option<String>,
    pub person: Option<Option<OID>>,
    pub group: Option<Option<OID>>,
    pub household: Option<Option<OID>>,
    pub key_type: Option<EventKeyType>,
}

impl COSIForm for EventRegistrationImpl {}
impl COSIForm for EventRegistrationOptional {}

impl From<EventRegistration> for EventRegistrationImpl {
    fn from(er: EventRegistration) -> EventRegistrationImpl {
        EventRegistrationImpl {
            event: <OID as std::default::Default>::default(),
            timestamp: "".to_string(),
            person: None,
            group: None,
            household: None,
            key_type: EventKeyType::Group,
        }
    }
}

impl From<EventRegistrationImpl> for EventRegistration {
    fn from(gr: EventRegistrationImpl) -> EventRegistration {
        EventRegistration {
            event: <Event as std::default::Default>::default(),
            timestamp: NaiveDate::from_ymd(2020, 6, 7).and_hms(7, 7, 7),
            person: None,
            group: None,
            household: None,
            key_type: EventKeyType::Group,
        }
    }
}

#[async_trait]
impl COSICollection<'_, EventRegistration, EventRegistrationImpl, EventRegistrationOptional>
    for EventRegistration
{
    fn get_table_name() -> String {
        return "eventregistration".to_string();
    }

    async fn to_impl(
        client: &Client,
        mut orm: Vec<EventRegistration>,
    ) -> COSIResult<Vec<EventRegistrationImpl>> {
        let collection = Self::get_collection(client).await;

        let event_raw = Event::get_raw_document(client).await;
        let group_raw = Person::get_raw_document(client).await;
        let house_raw = Household::get_raw_document(client).await;
        let person_raw = Person::get_raw_document(client).await;

        let mut final_results = Vec::new();
        let get_id = |d: &Document| -> OID { d.get("_id").unwrap().as_object_id().unwrap().into() };
        for o in &orm {
            let event_impl = Event::to_impl(client, vec![o.event.clone()]).await?[0].clone();
            let event = Event::find_document(client, Some(to_document(&event_impl)?), None)
                .await?
                .pop()
                .unwrap();
            let mut er_result = EventRegistrationImpl {
                event: get_id(&event),
                timestamp: o.timestamp.to_string(),
                person: None,
                group: None,
                household: None,
                key_type: o.key_type,
            };

            match &o.key_type {
                &EventKeyType::Group => {
                    let group_impl =
                        Group::to_impl(client, vec![o.group.clone().unwrap()]).await?[0].clone();
                    let group = Group::find_document(client, Some(to_document(&group_impl)?), None)
                        .await?
                        .pop()
                        .unwrap();
                    er_result.group = Some(get_id(&group));
                }
                &EventKeyType::Household => {
                    let house_impl = Household::to_impl(client, vec![o.household.clone().unwrap()])
                        .await?[0]
                        .clone();
                    let house =
                        Household::find_document(client, Some(to_document(&house_impl)?), None)
                            .await?
                            .pop()
                            .unwrap();
                    er_result.household = Some(get_id(&house));
                }
                &EventKeyType::Person => {
                    let person = person_raw
                        .find_one(to_document(&o.person)?, None)
                        .await?
                        .ok_or(COSIError::msg("Unable to fetch person table."))?;
                    er_result.person = Some(get_id(&person));
                }
            }

            final_results.push(er_result);
        }

        return Ok(final_results);
    }

    async fn to_orm(
        client: &Client,
        imp: &Vec<EventRegistrationImpl>,
    ) -> COSIResult<Vec<EventRegistration>> {
        let mut result = vec![];

        let person_col = Person::get_collection(client).await;
        let group_col = Group::get_collection(client).await;
        let household_col = Household::get_collection(client).await;
        let event_col = Event::get_collection(client).await;
        for i in imp {
            let event = event_col
                .find_one(doc! {"_id": ObjectId::from(i.event.clone())}, None)
                .await?
                .ok_or(COSIError::msg("Unable to find provided event."))?;
            let e_orm = Event::to_orm(client, &vec![event]).await?[0].clone();
            let mut er_result = EventRegistration {
                event: e_orm,
                timestamp: NaiveDateTime::parse_from_str(&i.timestamp, "%Y-%m-%d %H:%M:%S")?,
                person: None,
                group: None,
                household: None,
                key_type: i.key_type,
            };

            match &er_result.key_type {
                &EventKeyType::Group => {
                    // Inefficient conversion.
                    let group = group_col
                        .find_one(
                            doc! {"_id": ObjectId::from(i.group.as_ref().unwrap().clone())},
                            None,
                        )
                        .await?
                        .ok_or(COSIError::msg("Unable to find Group Event."))?;
                    let g_orm = Group::to_orm(client, &vec![group]).await?[0].clone();
                    er_result.group = Some(g_orm);
                }
                &EventKeyType::Household => {
                    // Inefficient conversion.
                    let household = household_col
                        .find_one(
                            doc! {"_id": ObjectId::from(i.household.as_ref().unwrap().clone())},
                            None,
                        )
                        .await?
                        .ok_or(COSIError::msg("Unable to find Household Event."))?;
                    let h_orm = Household::to_orm(client, &vec![household]).await?[0].clone();
                    er_result.household = Some(h_orm);
                }
                &EventKeyType::Person => {
                    // Inefficient conversion.
                    let person = person_col
                        .find_one(
                            doc! {"_id": ObjectId::from(i.person.as_ref().unwrap().clone())},
                            None,
                        )
                        .await?
                        .ok_or(COSIError::msg("Unable to find Person Event."))?;
                    let p_orm = Person::to_orm(client, &vec![person]).await?[0].clone();
                    er_result.person = Some(p_orm);
                }
            }

            result.push(er_result);
        }
        return Ok(result);
    }

    async fn process_foreign_keys<'b>(client: &'b Client, raw_doc: &'b mut Vec<Document>) {
        let impls: Vec<EventRegistrationImpl> = raw_doc
            .iter()
            .map(|x| from_document(x.clone()).unwrap())
            .collect();

        let orms: Vec<EventRegistration> = Self::to_orm(client, &impls).await.unwrap();
        let it = raw_doc.iter_mut().zip(orms);
        for (rd, o) in it {
            rd.insert("event", to_bson(&o.event).unwrap());
            rd.insert("group", to_bson(&None::<Group>).unwrap());
            rd.insert("household", to_bson(&None::<Household>).unwrap());
            rd.insert("person", to_bson(&None::<Person>).unwrap());

            match &o.key_type {
                &EventKeyType::Group => {
                    rd.insert("group", to_bson(&Some(&o.group)).unwrap());
                }
                &EventKeyType::Household => {
                    rd.insert("household", to_bson(&Some(&o.household)).unwrap());
                }
                &EventKeyType::Person => {
                    rd.insert("person", to_bson(&Some(&o.person)).unwrap());
                }
            }
        }
    }
}

#[async_trait]
impl Generator<EventRegistration> for EventRegistration {
    async fn generate(client: &Client, size: u32) -> COSIResult<Vec<EventRegistration>> {
        let mut result = Vec::new();

        let event_col = Event::get_collection(client).await;
        let person_col = Person::get_collection(client).await;
        let group_col = Group::get_collection(client).await;
        let household_col = Household::get_collection(client).await;

        let person_size = size / 3;
        let group_size = size / 3;
        let household_size = size - (person_size + group_size);

        let event_agg = event_col
            .aggregate([doc! {"$sample": {"size": size}}], None)
            .await?;
        let person_agg = person_col
            .aggregate([doc! {"$sample": {"size": person_size}}], None)
            .await?;
        let group_agg = group_col
            .aggregate([doc! {"$sample": {"size": group_size}}], None)
            .await?;
        let household_agg = household_col
            .aggregate([doc! {"$sample": {"size": household_size}}], None)
            .await?;

        // TODO: This can be parallelized.
        let result_event_doc: Vec<Document> = event_agg.try_collect().await.unwrap();
        let result_event_impl: Vec<EventImpl> = result_event_doc
            .iter()
            .map(|o| from_document(o.clone()).unwrap())
            .collect();
        let mut result_event: Vec<Event> = Event::to_orm(client, &result_event_impl).await.unwrap();

        let result_person_doc: Vec<Document> = person_agg.try_collect().await.unwrap();
        let result_person_impl: Vec<PersonImpl> = result_person_doc
            .iter()
            .map(|o| from_document(o.clone()).unwrap())
            .collect();
        let mut result_person: Vec<Person> =
            Person::to_orm(client, &result_person_impl).await.unwrap();

        let result_group_doc: Vec<Document> = group_agg.try_collect().await.unwrap();
        let result_group_impl: Vec<GroupImpl> = result_group_doc
            .iter()
            .map(|o| from_document(o.clone()).unwrap())
            .collect();
        let mut result_group: Vec<Group> = Group::to_orm(client, &result_group_impl).await.unwrap();

        let result_household_doc: Vec<Document> = household_agg.try_collect().await.unwrap();
        let result_household_impl: Vec<HouseholdImpl> = result_household_doc
            .iter()
            .map(|o| from_document(o.clone()).unwrap())
            .collect();
        let mut result_household: Vec<Household> =
            Household::to_orm(client, &result_household_impl)
                .await
                .unwrap();

        let mut rng = thread_rng();
        let gen_date = |rng: &mut ThreadRng| {
            NaiveDate::from_ymd(2022, rng.gen_range(1, 12), rng.gen_range(1, 28)).and_hms(7, 7, 7)
        };
        for _ in 0..person_size {
            result.push(EventRegistration {
                event: result_event.pop().unwrap(),
                timestamp: gen_date(&mut rng),
                person: Some(result_person.pop().unwrap()),
                group: None,
                household: None,
                key_type: EventKeyType::Person,
            });
        }

        for _ in 0..group_size {
            result.push(EventRegistration {
                event: result_event.pop().unwrap(),
                timestamp: gen_date(&mut rng),
                person: None,
                group: Some(result_group.pop().unwrap()),
                household: None,
                key_type: EventKeyType::Group,
            });
        }

        for _ in 0..household_size {
            result.push(EventRegistration {
                event: result_event.pop().unwrap(),
                timestamp: gen_date(&mut rng),
                person: None,
                group: None,
                household: Some(result_household.pop().unwrap()),
                key_type: EventKeyType::Household,
            });
        }

        return Ok(result);
    }
}
