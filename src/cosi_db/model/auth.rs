// Dealing with authentication techniques.
use crate::cosi_db::connection::COSIMongo;
use crate::cosi_db::controller::auth::Credential;
use crate::cosi_db::errors::COSIError;
use crate::cosi_db::model::common::{COSICollection, COSIForm, OID};

use rocket::form::FromForm;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub token: String,
}

#[derive(Clone, Debug, FromForm, Serialize, Deserialize)]
pub struct UserForm {
    pub username: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
}

impl COSIForm for User {}
impl COSIForm for UserForm {}

impl COSICollection<'_, User, User, UserForm> for User {
    fn get_table_name() -> String {
        return "user".to_string();
    }
}

// For security, logging items are in a separate table.
#[derive(Clone, Serialize, Deserialize)]
pub struct UserLogin {
    pub user_id: OID,
    pub password: Credential,
}

impl COSIForm for UserLogin {}

impl COSICollection<'_, UserLogin, UserLogin, UserLogin> for UserLogin {
    fn get_table_name() -> String {
        return "userlogin".to_string();
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = COSIError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<User, COSIError> {
        // Example from docs
        // https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html
        let docs: &Vec<User> = request
            .local_cache_async(async {
                let connect = request.guard::<&COSIMongo>().await.succeeded().unwrap();
                let client = &*connect;
                let uid: Option<String> = request
                    .cookies()
                    .get_private("user_id")
                    .and_then(|cookie| cookie.value().parse().ok());
                let token: String = request
                    .cookies()
                    .get_private("user_token")
                    .and_then(|cookie| cookie.value().parse().ok())
                    .unwrap_or("".to_string());
                match uid {
                    None => Vec::new(),
                    Some(ref v) => {
                        let search_doc = Some(doc! {
                            "_id": ObjectId::parse_str(&v).unwrap(),
                            "token": token
                        });
                        // TODO: Connection error handling.
                        User::find_data(client, search_doc, None).await.unwrap()
                    }
                }
            })
            .await;

        if docs.len() == 0 {
            Outcome::Forward(())
        } else if docs.len() > 1 {
            Outcome::Failure((
                Status::InternalServerError,
                COSIError::msg("Invalid login detected."),
            ))
        } else {
            Outcome::Success(docs[0].clone())
        }
    }
}
