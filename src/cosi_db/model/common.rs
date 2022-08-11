use async_trait::async_trait;
use mongodb::{
    bson::to_document, bson::Bson, bson::Document, options::FindOptions, options::InsertOneOptions,
};
use mongodb::{Collection, Cursor};

use futures::stream::TryStreamExt;

use crate::cosi_db::controller::common::get_connection;
use crate::cosi_db::errors::{COSIError, COSIResult};
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait Generator<T> {
    async fn generate(size: u32) -> COSIResult<Vec<T>>;
}

pub trait COSIForm {
    // Helper function to convert object to document.
    // Argument strict decides if all keys must be present.
    fn convert_to_document(&self, strict: bool) -> COSIResult<Document>
    where
        Self: Serialize,
    {
        let d = to_document(&self).unwrap();
        let mut result = Document::new();
        for v in d {
            match v.1 {
                Bson::Null => {
                    if strict {
                        return Err(COSIError::msg(format! {"{} key missing.", v.0}));
                    }
                }
                _ => {
                    result.insert(v.0, v.1);
                }
            }
        }
        return Ok(result);
    }

    fn sanitize_query(&self) -> COSIResult<Document>
    where
        Self: Serialize,
    {
        // We only want to search values that are not-null.
        // Double wrap in Option to allow for searching of nullable.
        self.convert_to_document(false)
    }

    fn sanitize_insert(&self) -> COSIResult<Document>
    where
        Self: Serialize,
    {
        // Insert does not allow for non-null.
        // Double wrap in Option to allow for searching of nullable.
        self.convert_to_document(true)
    }
}

#[async_trait]
pub trait COSICollection<'a, T, I, F>
where
    for<'r> T: Clone + Sized + Serialize + DeserializeOwned + Unpin + Send + Sync + From<I> + 'r, // Base class
    for<'r> I: Clone + Sized + Serialize + DeserializeOwned + Unpin + Send + Sync + From<T> + 'r,
    for<'r> F: Clone + Sized + Serialize + DeserializeOwned + Unpin + Send + COSIForm + 'r,
{
    fn get_table_name() -> String;
    async fn get_raw_document() -> Collection<Document> {
        let tname = Self::get_table_name();
        return get_connection(&tname).await.collection::<Document>(&tname);
    }

    async fn get_collection() -> Collection<I> {
        let tname = Self::get_table_name();
        get_connection(&tname).await.collection::<I>(&tname)
    }

    async fn to_impl(orm: Vec<T>) -> COSIResult<Vec<I>> {
        // This extra call allows for async side-effects.
        // Default implementation is non-bulk. Can be slow.
        Ok(orm.iter().map(|v| v.clone().into()).collect())
    }

    async fn to_orm(imp: Vec<I>) -> COSIResult<Vec<T>> {
        // This extra call allows for async side-effects.
        // Default implementation is non-bulk. Can be slow.
        Ok(imp.iter().map(|v| v.clone().into()).collect())
    }

    // Find with some extra processing for associated tables.
    async fn find_data(
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> COSIResult<Vec<T>> {
        let col = Self::get_collection().await;
        let cursor: Cursor<I> = col.find(filter, options).await?;
        let results = cursor.try_collect().await?;
        return Ok(Self::to_orm(results).await?);
    }

    async fn insert_datum(data: &I, options: Option<InsertOneOptions>) -> COSIResult<Bson> {
        let col = Self::get_collection().await;
        let result = col.insert_one(data, options).await?;
        return Ok(result.inserted_id);
    }

    // Used for processing formdata and input to internal representation.
    // This function technically doesn't need to be here as it is just a softwrapper
    // to into() however it allows for code-readers to understand the relationship between
    // Struct AImpl and Struct AForm.
    fn convert_form_query(form_data: F) -> COSIResult<Document> {
        return form_data.sanitize_query();
    }

    fn convert_form_insert(form_data: F) -> COSIResult<Document> {
        return form_data.sanitize_insert();
    }
}
