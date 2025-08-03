use std::path::{Path, PathBuf};
use crate::api::{save_avatar, CommonMessage};
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{get, web};
use actix_web::{post, Responder};
use serde::{Deserialize, Serialize};
use sn_internal::db::db_character::Character;
use sn_internal::db::{db_character, DBPool};
use crate::ConfigData;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/character")
            .service(search)
            .service(get)
            .service(delete)
            .service(update),
    );
}

#[derive(MultipartForm)]
struct CharacterForm {
    id: Text<i64>,
    username: Text<String>,
    name: Text<String>,
    avatar: Option<TempFile>,
}

impl Into<Character> for CharacterForm {
    fn into(self) -> Character {
        Character {
            id: self.id.into_inner(),
            username: self.username.into_inner(),
            name: self.name.into_inner(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct SearchQuery {
    #[serde(default)]
    search: String,
}

#[derive(Serialize)]
struct CharacterResponse {
    characters: Vec<Character>,
    err: String,
}

#[get("")]
async fn search(db_pool: web::Data<DBPool>, queries: web::Query<SearchQuery>) -> impl Responder {
    let mut characters = Vec::new();
    let mut err = String::new();
    match db_character::search(&db_pool, queries.search.as_str()).await {
        Ok(c) => characters = c,
        Err(e) => err = e.to_string(),
    }
    web::Json(CharacterResponse { characters, err })
}

#[get("/{id}")]
async fn get(db_pool: web::Data<DBPool>, id: web::Path<i64>) -> impl Responder {
    let mut characters = Vec::new();
    let mut err = String::new();
    match db_character::get(&db_pool, id.into_inner()).await {
        Ok(c) => characters.push(c),
        Err(e) => err = e.to_string(),
    }
    web::Json(CharacterResponse { characters, err })
}

#[get("delete/{id}")]
async fn delete(db_pool: web::Data<DBPool>, id: web::Path<i64>) -> impl Responder {
    let res = if let Err(e) = db_character::delete(&db_pool, id.into_inner()).await {
        CommonMessage::from_err(e.to_string())
    } else {
        CommonMessage::from_msg(String::from("Success"))
    };

    web::Json(res)
}

#[post("")]
async fn update(db_pool: web::Data<DBPool>, config_data: web::Data<ConfigData>, MultipartForm(data): MultipartForm<CharacterForm>) -> impl Responder {
    let mut err = String::new();
    let mut msg = String::new();
    let config = config_data.config.read().await;

    let file_name = format!("{}.png", data.username.0.clone().as_str());
    let data_dir = PathBuf::from(&config.data_dir);
    save_avatar(&data_dir, data.avatar, file_name.as_str()).await;

    let character: Character = data.into();
    if character.id > 0 {
        if let Err(e) = db_character::update(&db_pool, &character).await {
            err = e.to_string();
        } else {
            msg = "Success".to_string();
        }
    } else if let Err(e) = db_character::insert(&db_pool, &character).await {
        err = e.to_string();
    } else {
        msg = "Success".to_string();
    }


    web::Json(CommonMessage::new(msg, err))
}
