// Dealing with authentication techniques.
use crate::cosi_db::errors::COSIError;
use rocket::request::{FromRequest, Request};

#[derive(Copy, Clone)]
pub struct User {
    pub name: String,
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for User {
//     async fn from_request(req: &'r Request<'_>) -> Outcome<User, CosiError> {
//         // Example from docs
//         // https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html
//         let user_result = request.local_cache_async(async {
//             request.cookies()
//                 .get_private("user_id")
//                 .and_then(|cookie| cookie.value().parse().ok())
//                 .and_then(|id| db.get_user(id).ok())
//                 .or_forward(CosiError::msg("User identication cookie issues. Have you considered clearing cookies?"))
//         }).await;
//     }
// }
