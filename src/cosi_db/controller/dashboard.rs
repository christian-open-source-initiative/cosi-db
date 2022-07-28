use rocket::response::content::RawHtml;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> RawHtml<Template> {
    RawHtml(Template::render("dashboard", context! {}))
}
