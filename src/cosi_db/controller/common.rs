use serde::{Deserialize, Serialize};

// cosi_db
use crate::cosi_db::connection::{CosiDB, MongoConnection};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginateData<T> {
    pub page: u64,
    pub total_pages: u64,
    pub data: Vec<T>,
}

pub async fn get_connection() -> CosiDB {
    CosiDB::new("admin", "admin", None).await.unwrap()
}

// Helper macros to generate endpoints.
// Use paste to auto-generate a helper macro.
#[macro_export]
macro_rules! generate_pageable_getter {
    ($T:ident, $S:literal) => {
        $crate::paste::paste! {
            $crate::with_builtin_macros::with_builtin!{
                let $v_path = concat!("/get_", $S, "?<page>") in {
                    #[get($v_path)]
                    pub async fn [<get_ $T:lower>](page: Option<u64>) -> RawJson<String> {
                        let page = page.unwrap_or(0);

                        let col = $crate::cosi_db::controller::common::get_connection().await
                            .client
                            .database("cosi_db")
                            .collection::<$T>(&stringify!($T).to_lowercase());

                        // Page calculate.
                        let total_result: u64 = col.estimated_document_count(None).await.unwrap();
                        let limit_size: i64 = 100;
                        let total_pages: u64 = (total_result as f64 / limit_size as f64).ceil() as u64;

                        let find_options = FindOptions::builder()
                            .limit(limit_size)
                            .skip(limit_size as u64 * page)
                            .build();
                        let data_cursor = col.find(doc! {}, Some(find_options)).await.unwrap();
                        let data: Vec<$T> = data_cursor.try_collect().await.unwrap();

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
