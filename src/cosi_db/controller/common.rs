use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginateData<T> {
    pub page: u64,
    pub total_pages: u64,
    pub data: Vec<T>,
}

// Helper macros to generate endpoints.
// Use paste to auto-generate a helper macro.
#[macro_export]
macro_rules! generate_pageable_getter {
    ($table: ident, $route: expr) => {
        use paste::paste;
        paste! {
            #[get([</get $table _{} ?<page>>])]
            pub fn [<get $table:lower>](page: Option<u64>) -> RawJson<String> {
                let page = page.unwrap_or(0);

                let connection = get_connection().await;
                let col = connection
                    .client
                    .database("cosi_db")
                    .collection::<$table>(stringify!($ident).to_lower());

                // Page calculate.
                let total_result: u64 = col.estimated_document_count(None).await.unwrap();
                let batch_size: u32 = 100;
                let total_pages: u64 = (total_result as f64 / batch_size as f64).ceil() as u64;

                let find_options = FindOptions::builder()
                    .batch_size(batch_size)
                    .skip(batch_size as u64 * page)
                    .build();
                let data_cursor = col.find(doc! {}, Some(find_options)).await.unwrap();
                let data: Vec<Address> = data_cursor.try_collect().await.unwrap();

                RawJson(
                    serde_json::to_string(&PaginateData {
                        page: page,
                        total_pages: total_pages,
                        data: data,
                    })
                    .unwrap(),
                )
            }
        }
    };
}
