// Handles connection to database.
use async_trait::async_trait;

use mongodb::{
    bson::doc,
    options::ClientOptions, Client
};

pub struct CosiDBConfigs {
    pub ip: String,
    pub port: String,
    pub db_name: String 
}

impl Default for CosiDBConfigs {
    fn default() -> Self {
        CosiDBConfigs {
            ip: "localhost".to_string(),
            port: "27017".to_string(),
            db_name: "cosi_db".to_string()
        }
    }
}

pub struct CosiDB {
    pub client: Client
}

#[async_trait]
pub trait MongoConnection {
    async fn new(
        user: &str,
        pass: &str,
        config: Option<CosiDBConfigs>) -> mongodb::error::Result<CosiDB>;
}

#[async_trait]
impl MongoConnection for CosiDB {
    async fn new(
        user: &str,
        pass: &str,
        config: Option<CosiDBConfigs>
    ) -> mongodb::error::Result<CosiDB> {
        let c = config.unwrap_or_default();

        let mut client_options = ClientOptions::parse(
            format!("mongodb://{}:{}@{}:{}/{}", user, pass, c.ip, c.port, c.db_name)
        ).await?;

        client_options.app_name =  Some("cosi_db".to_string());
        let client = Client::with_options(client_options)?;
        Ok(CosiDB { client })
    }
}
