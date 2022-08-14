// Handles connection to database.
use rocket_db_pools::Database;

#[derive(Database)]
#[database("mongodb")]
pub struct COSIMongo(mongodb::Client);
