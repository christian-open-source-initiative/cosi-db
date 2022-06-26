// Handles connection to database.
use async_trait::async_trait;

use mongodb::{
    bson::doc,
    options::ClientOptions, Client
};

pub struct CosiDB {
    client: Client
}

#[async_trait]
pub trait MongoConnection {
    async fn new(url: &'static str) -> mongodb::error::Result<CosiDB>;
}

#[async_trait]
impl MongoConnection for CosiDB {
    async fn new(url: &'static str) -> mongodb::error::Result<CosiDB> {
        let mut client_options = ClientOptions::parse(url).await?;
        client_options.app_name =  Some("cosi_db".to_string());
        let client = Client::with_options(client_options)?;

        client.database("cosi_db").run_command(doc! {"ping": 1}, None).await?;
        Ok(CosiDB { client })
    }
}
