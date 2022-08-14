use crate::cosi_db::model::auth::*;
use rocket_dyn_templates::Template;

#[get("/login", rank = 3)]
pub fn login() -> Template {
    Template::render("login", context! {})
}

#[get("/login", rank = 2)]
pub fn login_logged(user: User) -> &'static str {
    "You are already logged in."
}
