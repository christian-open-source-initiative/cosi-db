use async_trait::async_trait;
use mongodb::{bson::Document, options::FindOptions};
use mongodb::{Collection, Cursor};

use futures::stream::TryStreamExt;

use crate::cosi_db::controller::common::get_connection;
use crate::cosi_db::errors::COSIResult;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait Generator<T> {
    async fn generate(size: u32) -> COSIResult<Vec<T>>;
}

#[async_trait]
pub trait COSICollection<'a, T, I, F>
where
    for<'r> T: Clone + Sized + Serialize + DeserializeOwned + Unpin + Send + Sync + From<I> + 'r, // Base class
    for<'r> I:
        Clone + Sized + Serialize + DeserializeOwned + Unpin + Send + Sync + From<T> + From<F> + 'r,
    for<'r> F: Clone + Sized + Serialize + DeserializeOwned + Unpin + Send + 'r,
{
    fn get_table_name() -> String;
    async fn get_raw_document() -> Collection<Document> {
        return get_connection()
            .await
            .collection::<Document>(&Self::get_table_name());
    }

    async fn get_collection() -> Collection<I>;

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

    // Used for processing formdata and input to internal representation.
    // This function technically doesn't need to be here as it is just a softwrapper
    // to into() however it allows for code-readers to understand the relationship between
    // Struct AImpl and Struct AForm.
    fn convert_form_input(form_data: F) -> COSIResult<I> {
        return Ok(form_data.into());
    }
}
