use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpResponse, Responder};
use std::path::PathBuf;

use crate::db;
use crate::route::{redirect, save_file};
use crate::AppState;

#[derive(MultipartForm)]
struct CharacterForm {
    username: Text<String>,
    name: Text<String>,
    avatar: Option<TempFile>,
}

#[get("/characters")]
pub async fn characters(data: web::Data<AppState>, tmpl: web::Data<tera::Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();

    if let Ok(characters) = db::character::get_all(&data.pool).await {
        ctx.insert("characters", &characters);
    }

    let template = tmpl.render("characters.gohtml", &ctx).unwrap_or("Not found".into());
    HttpResponse::Ok().content_type("text/html").body(template)
}

#[get("/character/{username}")]
pub async fn character(
    data: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
    username: web::Path<String>,
) -> impl Responder {
    let mut ctx = tera::Context::new();

    if let Ok(char) = db::character::get(&data.pool, &username.into_inner()).await {
        ctx.insert("character", &char);
    }

    let template = tmpl.render("character.gohtml", &ctx).unwrap_or("Not found".into());
    HttpResponse::Ok().content_type("text/html").body(template)
}

#[post("/add/character")]
pub async fn add_character(
    data: web::Data<AppState>,
    MultipartForm(form): MultipartForm<CharacterForm>,
) -> impl Responder {
    if let Err(e) = db::character::add(&data.pool, form.username.0.as_str(), form.name.0.as_str()).await {
        log::error!("Failed to add character {}: {}", &form.username.0, e);
        return redirect!("/characters");
    }

    let file_name = format!("{}.png", form.username.0.as_str());
    save_avatar(&data.root_dir, form.avatar, file_name.as_str());

    redirect!("/characters")
}

#[post("/update/character")]
pub async fn update_character(
    data: web::Data<AppState>,
    MultipartForm(form): MultipartForm<CharacterForm>,
) -> impl Responder {
    let url = format!("/character/{}", form.username.0);

    if let Err(e) = db::character::update(&data.pool, form.username.0.as_str(), form.name.0.as_str()).await {
        log::error!("Failed to update character {}: {}", form.username.0, e);
        return redirect!(url);
    }

    let file_name = format!("{}.png", form.username.0);
    save_avatar(&data.root_dir, form.avatar, file_name.as_str());

    redirect!(url)
}

#[get("/delete/character/{username}")]
pub async fn delete_character(data: web::Data<AppState>, username: web::Path<String>) -> impl Responder {
    let name = username.into_inner().clone();
    if let Err(e) = db::character::delete(&data.pool, name.clone().as_str()).await {
        log::error!("Failed to delete character {}: {}", name, e);
    }
    redirect!("/characters")
}

fn save_avatar(root_dir: &PathBuf, avatar: Option<TempFile>, file_name: &str) {
    let path = root_dir.join("avatar");
    if save_file(&path, avatar, file_name).is_ok() {
        log::info!("Saved avatar successfully");
    }
}
