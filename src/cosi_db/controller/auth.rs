use crate::cosi_db::connection::COSIMongo;
use crate::cosi_db::model::auth::*;
use crate::cosi_db::model::common::COSICollection;

use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;

use ring::digest::SHA256_OUTPUT_LEN;
use ring::pbkdf2;
use std::num::NonZeroU32;
use uuid::Uuid;

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::content::{RawHtml, RawJson};
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

// WARNING, changing things means re-hashing your passwords.
pub const CREDENTIAL_LEN: usize = SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

#[get("/login", rank = 3)]
pub fn login() -> RawHtml<Template> {
    RawHtml(Template::render("login", context! {}))
}

#[post("/login", data = "<user_form>")]
pub async fn login_submit(
    connect: Connection<COSIMongo>,
    cookies: &CookieJar<'_>,
    user_form: Form<UserForm>,
) -> RawJson<String> {
    // TODO: Move this to sanitize
    if user_form.token.is_none() {
        return RawJson("{err: \"Password not entered.\"}".to_string());
    }

    let client: &Client = &*connect;
    let user_form_obj: UserForm = user_form.into_inner();
    let find_doc = User::convert_form_query(user_form_obj.clone()).unwrap();
    let user_doc_opt = User::find_document(client, Some(find_doc), None).await;

    match user_doc_opt {
        Err(e) => {
            return RawJson(format!("{{err: {}}}", e));
        }
        Ok(d_vec) => {
            if d_vec.len() == 0 {
                return RawJson(format!("{{err: {}}}", "Invalid user or password."));
            } else if d_vec.len() > 1 {
                return RawJson(format!("{{err: {}}}", "Internal server error."));
            }

            let oid = d_vec[0].get("_id").unwrap().to_string();
            // TODO: Error check here.
            let u_login_doc =
                UserLogin::find_data(client, Some(doc! {"user_id": oid.clone() }), None).await;
            if let Err(_) = u_login_doc {
                return RawJson(format!("{{err: {}}}", "Internal server error."));
            }
            let u_logins: Vec<UserLogin> = u_login_doc.unwrap();
            if u_logins.len() > 1 || u_logins.len() == 0 {
                return RawJson(format!("{{err: {}}}", "Internal server error."));
            }

            let u_login = &u_logins[0];
            let u_login_user_id: ObjectId = u_login.user_id.clone().into();

            let db_oid = u_login_user_id.to_string();

            let mut salt = Vec::with_capacity(db_oid.len());
            salt.extend(db_oid.as_bytes());

            let mut calc_password: Credential = [0u8; CREDENTIAL_LEN];
            pbkdf2::derive(
                pbkdf2::PBKDF2_HMAC_SHA256,
                NonZeroU32::new(50000).unwrap(),
                &salt,
                user_form_obj.token.unwrap().clone().as_bytes(),
                &mut calc_password,
            );

            if calc_password != u_login.password {
                return RawJson(format!("{{err: {}}}", "Incorrect username or password."));
            }

            let uuid_str = Uuid::new_v4().to_string();

            // Update user token on DB to persist.
            let update_result = User::update_datum(
                client,
                &doc! {"_id": oid.clone()},
                &doc! {"token": uuid_str.clone()},
                None,
            )
            .await;
            if let Err(_) = update_result {
                return RawJson(format!("{{err: {}}}", "Internal server error."));
            }

            // Update cookies
            cookies.add_private(Cookie::new("user_id", db_oid.clone()));
            cookies.add_private(Cookie::new("user_token", uuid_str.clone()));
            return RawJson(format!("{{success: {}}}", "Logged In"));
        }
    }
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    cookies.remove_private(Cookie::named("user_token"));
    Flash::success(Redirect::to("/login"), "Logging out.")
}

#[get("/login", rank = 2)]
pub fn login_logged(_user: User) -> Redirect {
    Redirect::to(uri!("/"))
}
