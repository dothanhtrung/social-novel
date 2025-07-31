use crate::api::CommonMessage;
use actix_web::{get, web};
use actix_web::{post, Responder};
use serde::{Deserialize, Serialize};
use sn_internal::db::db_character::Character;
use sn_internal::db::{db_character, DBPool};

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/character")
            .service(search)
            .service(get)
            .service(delete)
            .service(update),
    );
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
async fn update(db_pool: web::Data<DBPool>, data: web::Json<Character>) -> impl Responder {
    let character = data.into_inner();
    let mut err = String::new();
    let mut msg = String::new();
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
