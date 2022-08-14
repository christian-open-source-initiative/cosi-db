use mongodb::{options::FindOptions, options::InsertOneOptions};
use rocket::async_trait;
use rocket::data::ToByteUnit;
use rocket::form::{DataField, FromFormField, ValueField};
use std::str::FromStr;

use mongodb::bson::{oid::ObjectId, to_document, Bson, Document};
use mongodb::{Client, Collection, Cursor};

use futures::stream::{StreamExt, TryStreamExt};

use crate::cosi_db::errors::COSIResult;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OID(pub ObjectId);

impl OID {
    pub fn vec_to_object_id(oid: &Vec<OID>) -> Vec<ObjectId> {
        oid.iter().map(|x| x.0).collect()
    }
}

impl Default for OID {
    fn default() -> Self {
        OID(ObjectId::default())
    }
}

impl From<ObjectId> for OID {
    fn from(oid: ObjectId) -> Self {
        OID(oid)
    }
}

impl From<OID> for ObjectId {
    fn from(oid: OID) -> Self {
        oid.0
    }
}

#[rocket::async_trait]
impl<'a> FromFormField<'a> for OID {
    fn from_value(field: ValueField<'a>) -> rocket::form::Result<'a, Self> {
        Ok(ObjectId::from_str(field.value).unwrap().into()) // TODO: Error handling.
    }

    async fn from_data(field: DataField<'a, '_>) -> rocket::form::Result<'a, Self> {
        let limit = field
            .request
            .limits()
            .get("_oid")
            .unwrap_or(256_i32.kibibytes());
        let bytes = field.data.open(limit).into_bytes().await?;
        if !bytes.is_complete() {
            Err((None, Some(limit)))?;
        }

        let bytes = bytes.into_inner();
        let bytes = rocket::request::local_cache!(field.request, bytes);
        Ok(ObjectId::from_str(std::str::from_utf8(bytes)?)
            .unwrap()
            .into()) // TODO: Error check.
    }
}

#[async_trait]
pub trait Generator<T> {
    async fn generate(client: &Client, size: u32) -> COSIResult<Vec<T>>;
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
                        result.insert(v.0, v.1);
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
    for<'r> I: Clone
        + Sized
        + Serialize
        + DeserializeOwned
        + Unpin
        + Send
        + Sync
        + From<T>
        + COSIForm
        + 'r,
    for<'r> F: Clone + Sized + Serialize + DeserializeOwned + Unpin + Send + Sync + COSIForm + 'r,
{
    fn get_table_name() -> String;
    async fn get_raw_document(client: &Client) -> Collection<Document> {
        let tname = Self::get_table_name();
        return client.database("cosi_db").collection::<Document>(&tname);
    }

    async fn get_collection(client: &Client) -> Collection<I> {
        let tname = Self::get_table_name();
        return client.database("cosi_db").collection::<I>(&tname);
    }

    async fn to_impl(client: &Client, orm: Vec<T>) -> COSIResult<Vec<I>> {
        // This extra call allows for async side-effects.
        // Default implementation is non-bulk. Can be slow.
        Ok(orm.iter().map(|v| v.clone().into()).collect())
    }

    async fn to_orm(client: &Client, imp: Vec<I>) -> COSIResult<Vec<T>> {
        // This extra call allows for async side-effects.
        // Default implementation is non-bulk. Can be slow.
        Ok(imp.iter().map(|v| v.clone().into()).collect())
    }

    // Find with some extra processing for associated tables.
    async fn find_data(
        client: &Client,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> COSIResult<Vec<T>> {
        let col = Self::get_collection(client).await;
        let cursor: Cursor<I> = col.find(filter, options).await?;
        let results = cursor.try_collect().await?;
        return Ok(Self::to_orm(client, results).await?);
    }

    async fn find_document(
        client: &Client,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> COSIResult<Vec<Document>> {
        let col = Self::get_raw_document(client).await;
        let mut cursor: Cursor<Document> = col.find(filter, options).await?;
        let mut results = Vec::new();
        while let Some(doc) = cursor.next().await {
            results.push(doc?);
        }
        return Ok(results);
    }

    async fn insert_datum(
        client: &Client,
        data: &I,
        options: Option<InsertOneOptions>,
    ) -> COSIResult<Bson> {
        let col = Self::get_collection(client).await;
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

    fn convert_form_insert(form_data: I) -> COSIResult<Document> {
        return form_data.sanitize_insert();
    }
}
