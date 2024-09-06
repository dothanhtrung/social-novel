use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::TryStreamExt;
use serde::Deserialize;

use crate::AppState;
use crate::route::redirect;
use crate::db::character;

#[derive(Deserialize, Default)]
struct UserForm {
    username: String,
    name: String,
}

#[get("/users")]
pub async fn users(data: web::Data<AppState>, tmpl: web::Data<tera::Tera>)-> impl Responder {
    let mut ctx = tera::Context::new();

    if let Ok(characters) = character::get_all_users(&data.pool).await {
        ctx.insert("characters", &characters);
    }

    let template = tmpl.render("characters.html", &ctx).unwrap_or("Not found".into());
    HttpResponse::Ok().content_type("text/html").body(template)
}

#[post("/add_user")]
pub async fn add_user(data: web::Data<AppState>, ) -> impl Responder {


    redirect!("/users")
}