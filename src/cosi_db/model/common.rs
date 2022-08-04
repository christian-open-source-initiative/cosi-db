use async_trait::async_trait;
use mongodb::{bson::Document, options::FindOptions};
use mongodb::{Collection, Cursor};

use core::fmt::Display;
use futures::stream::{StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::default::Default;
use std::iter::Extend;

#[async_trait]
pub trait Generator<T> {
    async fn generate(size: u32) -> Vec<T>;
}

#[async_trait]
pub trait COSICollection<'a, T>
where
    T: Sized + Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    async fn get_collection() -> Collection<T>;

    // Find with some extra processing for associated tables.
    async fn find(filter: Option<Document>, options: Option<FindOptions>) -> Vec<T> {
        let col = Self::get_collection().await;
        let cursor: Cursor<T> = col.find(filter, options).await.unwrap();
        return cursor.try_collect().await.unwrap();
    }
}
