use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::TryStreamExt;
use serde::Deserialize;

use crate::{AppState};
use crate::route::redirect;
use crate::db::character;

#[derive(MultipartForm)]
struct CharacterForm {
    username: Text<String>,
    name: Text<String>,
    avatar: Option<TempFile>,
}

#[get("/characters")]
pub async fn characters(data: web::Data<AppState>, tmpl: web::Data<tera::Tera>)-> impl Responder {
    let mut ctx = tera::Context::new();

    if let Ok(characters) = character::get_all_characters(&data.pool).await {
        ctx.insert("characters", &characters);
    }

    let template = tmpl.render("characters.html", &ctx).unwrap_or("Not found".into());
    HttpResponse::Ok().content_type("text/html").body(template)
}

#[post("/add_character")]
pub async fn add_character(data: web::Data<AppState>, MultipartForm(form): MultipartForm<CharacterForm>) -> impl Responder {
    let _ = character::add_character(&data.pool, form.username.0.as_str(), form.name.0.as_str()).await;
    redirect!("/characters")
}