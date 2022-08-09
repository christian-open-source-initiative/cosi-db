use mongodb::Database;
use serde::{Deserialize, Serialize};

// cosi_db
use crate::cosi_db::connection::{CosiDB, MongoConnection};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginateData<T> {
    pub page: u64,
    pub total_pages: u64,
    pub data: Vec<T>,
}

pub async fn get_connection() -> Database {
    CosiDB::new("admin", "admin", None)
        .await
        .unwrap()
        .client
        .database("cosi_db")
}

// Helper macros to generate endpoints.
// Use paste to auto-generate a helper macro.

// GENERATORS
#[macro_export]
macro_rules! generate_generators {
    ($T:ident) => {
        $crate::paste::paste! {
            $crate::with_builtin_macros::with_builtin!{
                let $v_path = concat!("/gen_", stringify!([<$T: lower>]),  "/<total>") in {
                    #[get($v_path)]
                    pub async fn [<generate_ $T:lower>](total: u8) -> RawJson<String> {

                        #[cfg(debug_assertions)]
                        {
                            let data = $T::generate(total as u32).await.unwrap();

                            let col = $T::get_collection().await;
                            col.drop(None).await.unwrap();
                            col.insert_many($T::to_impl(data).await.unwrap(), None).await.unwrap();

                            let total = col.estimated_document_count(None).await.unwrap();
                            return RawJson(format!("{{\"total\": {}}}", total));
                        }
                        #[cfg(not(debug_assertions))]
                        {
                            return RawJson("{}".to_string());
                        }
                    }
                }
            }
        }
    };
}

// GETTERS
#[macro_export]
macro_rules! generate_pageable_getter {
    ($T:ident) => {
        $crate::paste::paste! {
            $crate::with_builtin_macros::with_builtin!{
                let $v_path = concat!("/get_", stringify!([<$T: lower>]), "?<page>&<search_query..>") in {
                    #[get($v_path)]
                    pub async fn [<get_ $T:lower>](page: Option<u64>, search_query: [<$T Form>]) -> RawJson<String> {
                        let page = page.unwrap_or(0);

                        let col = $T::get_collection().await;

                        // Page calculate.
                        let total_result: u64 = col.estimated_document_count(None).await.unwrap();
                        let limit_size: i64 = 100;
                        let total_pages: u64 = (total_result as f64 / limit_size as f64).ceil() as u64;

                        let find_options = FindOptions::builder()
                            .limit(limit_size)
                            .skip(limit_size as u64 * page)
                            .build();

                        let search_doc = $T::convert_form_input(search_query).unwrap();
                        // Query any search_queries
                        let data: Vec<$T> = $T::find_data(Some(search_doc), Some(find_options)).await.unwrap();

                        RawJson(
                            serde_json::to_string(&PaginateData {
                                page: page,
                                total_pages: total_pages,
                                data: data
                            }).unwrap()
                        )
                    }
                }
            }
        }
    }
}
