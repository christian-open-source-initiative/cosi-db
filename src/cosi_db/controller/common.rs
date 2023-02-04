use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginateData<T> {
    pub page: u64,
    pub total_pages: u64,
    pub data: Vec<T>,
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
                    pub async fn [<gen_ $T:lower>](_user: User, connect: Connection<COSIMongo>, total: u8) -> RawJson<String> {
                        #[cfg(debug_assertions)]
                        {
                            let client: &Client = &*connect;
                            let data = $T::generate(client, total as u32).await.unwrap();

                            let col = $T::get_collection(client).await;
                            col.drop(None).await.unwrap();
                            col.insert_many($T::to_impl(client, data).await.unwrap(), None).await.unwrap();

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
                    pub async fn [<get_ $T:lower>](_user: User, connect: Connection<COSIMongo>, page: Option<u64>, search_query: [<$T Optional>]) -> RawJson<String> {
                        let client: &Client = &*connect;
                        let page = page.unwrap_or(0);

                        let col = $T::get_collection(client).await;

                        // Page calculate.
                        let search_doc = $T::convert_form_query(search_query).unwrap();
                        let total_result:u64 = if search_doc.len() != 0 {
                            col.count_documents(Some(search_doc.clone()), None).await.unwrap()
                        } else {
                            col.estimated_document_count(None).await.unwrap()
                        };

                        let limit_size: i64 = 100;
                        let total_pages: u64 = (total_result as f64 / limit_size as f64).ceil() as u64;

                        let find_options = FindOptions::builder()
                            .limit(limit_size)
                            .skip(limit_size as u64 * page)
                            .build();

                        // Query any search_queries
                        let data: Vec<Document> = $T::find_document(client, Some(search_doc), Some(find_options)).await.unwrap();
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

// INSERT
#[macro_export]
macro_rules! generate_pageable_inserter {
    ($T:ident) => {
        $crate::paste::paste! {
            $crate::with_builtin_macros::with_builtin!{
                let $v_path = concat!("/insert_", stringify!([<$T: lower>])) in {
                    #[post($v_path, data="<insert_query>")]
                    pub async fn [<insert_ $T:lower>](_user: User, connect: Connection<COSIMongo>, insert_query: Form<[<$T Impl>]>) -> Custom<RawJson<String>> {
                        let client: &Client = &*connect;
                        let insert_query_obj = insert_query.into_inner();
                        let search_convert = $T::convert_form_insert(insert_query_obj);
                        return match search_convert {
                            Ok(search_obj) => {
                                // Query any search_queries
                                let bson_id: Bson = $T::insert_datum(client, &from_document(search_obj).unwrap(), None).await.unwrap();
                                Custom(Status::Ok, RawJson(
                                    serde_json::to_string(&bson_id).unwrap()
                                ))
                            },
                            Err(err) => {
                                Custom(Status::BadRequest, RawJson(format!("{{\"err\": \"{}\"}}", err)))
                            }
                        }
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! generate_pageable_update {
    ($T:ident) => {
        $crate::paste::paste! {
            $crate::with_builtin_macros::with_builtin!{
                let $v_path = concat!("/update_", stringify!([<$T: lower>]), "?<oid>") in {
                    #[post($v_path, data="<update_query>")]
                    pub async fn [<update_ $T:lower>](_user: User, connect: Connection<COSIMongo>, oid: String, update_query: Form<HashMap<String, String>>) -> Custom<RawJson<String>> {
                        let client: &Client = &*connect;
                        // update_query has to be a type of HashMap as we would like the user to update
                        // to null (thus the form has to be nullable) but we don't want update if the form value is not present
                        // e.g. the value is not given. As a result, we cannot use FromForm auto-parsing.
                        let raw_data: HashMap<String, String> = update_query.into_inner();
                        let bson_data = to_bson(&raw_data).unwrap();
                        println!("{:?}", bson_data);
                        let bson_id: Option<Bson> = $T::update_datum(client, &doc!{"_id": oid.clone()}, &doc!{"$set": bson_data}, None).await.unwrap();
                        Custom(Status::Ok, RawJson(
                            serde_json::to_string(&bson_id).unwrap()
                        ))
                    }
                }
            }
        }
    }
}

// DROP
#[macro_export]
macro_rules! generate_dropper {
    ($T:ident) => {
        $crate::paste::paste! {
            $crate::with_builtin_macros::with_builtin!{
                let $v_path = concat!("/drop_", stringify!([<$T: lower>])) in {
                    #[get($v_path)]
                    pub async fn [<drop_ $T:lower>](_user:User, connect: Connection<COSIMongo>) -> RawJson<String> {
                        #[cfg(debug_assertions)]
                        {
                            let client: &Client = &*connect;
                            let col = $T::get_collection(client).await;
                            col.drop(None).await.unwrap();
                            $T::create_collection(client).await.unwrap();
                            return RawJson(format!("{{\"dropped\": true}}"));
                        }
                        #[cfg(not(debug_assertions))]
                        {
                            return RawJson("{}".to_string());
                        }
                    }
                }
            }
        }
    }
}
