use crate::{delete_file, save_avatar, save_file_overwrite, CommonMessage};
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, Responder};
use my_config::ConfigData;
use my_db::db_ads::{self, Ads, AdsCond};
use my_db::db_character::{self, Character};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::error;
use web_misc::db::DBPool;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .service(
                web::scope("/user")
                    .service(user_search)
                    .service(user_get)
                    .service(user_delete)
                    .service(user_update),
            )
            .service(
                web::scope("/sponsored")
                    .service(sponsored_search)
                    .service(sponsored_get)
                    .service(sponsored_delete)
                    .service(sponsored_update),
            ),
    );
}

fn parse_bool(value: &str) -> bool {
    matches!(value.trim().to_ascii_lowercase().as_str(), "true" | "1" | "on" | "yes")
}

fn is_real_upload(temp_file: &TempFile) -> bool {
    temp_file.size > 0 && temp_file.file_name.as_deref().is_some_and(|name| !name.is_empty())
}

fn upload_extension(temp_file: &TempFile) -> String {
    Path::new(temp_file.file_name.as_deref().unwrap_or(""))
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "bin".to_string())
}

fn public_media_url(rel: &str) -> String {
    if rel.is_empty() {
        String::new()
    } else if rel.starts_with('/') || rel.starts_with("http") {
        rel.to_string()
    } else {
        format!("/data/{rel}")
    }
}

fn sponsored_view(ads: Ads) -> SponsoredItem {
    SponsoredItem {
        id: ads.id,
        description: ads.description,
        active: ads.active,
        media: public_media_url(&ads.media),
    }
}

#[derive(MultipartForm)]
struct UserForm {
    id: Text<i64>,
    username: Text<String>,
    name: Text<String>,
    avatar: Option<TempFile>,
}

#[derive(Deserialize)]
struct UserQuery {
    #[serde(default)]
    search: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserResponse {
    users: Vec<Character>,
    err: String,
}

#[get("")]
async fn user_search(db_pool: web::Data<DBPool>, queries: web::Query<UserQuery>) -> impl Responder {
    let mut users = Vec::new();
    let mut err = String::new();
    let cond = db_character::CharacterCond {
        room: None,
        search: if queries.search.is_empty() {
            None
        } else {
            Some(queries.search.clone())
        },
    };
    match db_character::search(&db_pool, &cond).await {
        Ok(c) => users = c,
        Err(e) => err = e.to_string(),
    }
    web::Json(UserResponse { users, err })
}

#[get("/{id}")]
async fn user_get(db_pool: web::Data<DBPool>, id: web::Path<i64>) -> impl Responder {
    let mut users = Vec::new();
    let mut err = String::new();
    match db_character::get(&db_pool, id.into_inner()).await {
        Ok(c) => users.push(c),
        Err(e) => err = e.to_string(),
    }
    web::Json(UserResponse { users, err })
}

#[get("/delete/{id}")]
async fn user_delete(
    db_pool: web::Data<DBPool>,
    config_data: web::Data<ConfigData>,
    id: web::Path<i64>,
) -> impl Responder {
    let user_id = id.into_inner();
    let config = config_data.config.read().await;
    let data_dir = PathBuf::from(&config.data_dir);

    if let Ok(user) = db_character::get(&db_pool, user_id).await {
        delete_file(&data_dir, &format!("avatar/{}.png", user.username)).await;
    }

    let res = if let Err(e) = db_character::delete(&db_pool, user_id).await {
        CommonMessage::from_err(e.to_string())
    } else {
        CommonMessage::from_msg(String::from("Success"))
    };
    web::Json(res)
}

#[post("")]
async fn user_update(
    db_pool: web::Data<DBPool>,
    config_data: web::Data<ConfigData>,
    MultipartForm(data): MultipartForm<UserForm>,
) -> impl Responder {
    let mut err = String::new();
    let mut msg = String::new();
    let config = config_data.config.read().await;
    let data_dir = PathBuf::from(&config.data_dir);
    let username = data.username.0.trim().to_string();
    let name = data.name.0.trim().to_string();
    let id = data.id.0;

    if username.is_empty() || name.is_empty() {
        return web::Json(CommonMessage::from_err("Username and name are required".to_string()));
    }

    let mut description = String::new();
    if id > 0 {
        if let Ok(old) = db_character::get(&db_pool, id).await {
            description = old.description.clone();
            if old.username != username {
                let old_avatar = data_dir.join("avatar").join(format!("{}.png", old.username));
                let new_avatar = data_dir.join("avatar").join(format!("{username}.png"));
                if old_avatar.exists() && !new_avatar.exists() && data.avatar.is_none() {
                    if let Err(e) = fs::rename(&old_avatar, &new_avatar).await {
                        error!("Failed to rename avatar {} -> {}: {}", old.username, username, e);
                    }
                } else if old_avatar.exists() {
                    delete_file(&data_dir, &format!("avatar/{}.png", old.username)).await;
                }
            }
        }
    }

    let file_name = format!("{username}.png");
    if let Some(avatar) = data.avatar {
        save_avatar(&data_dir, avatar, file_name.as_str()).await;
    }

    let character = Character {
        username,
        name,
        id,
        description,
        bio: Json::default(),
    };
    if character.id > 0 {
        if let Err(e) = db_character::update(&db_pool, &character).await {
            err = e.to_string();
        } else {
            msg = "Success".to_string();
        }
    } else if let Err(e) = db_character::insert(&db_pool, &character).await {
        err = e.to_string();
        error!("Failed to insert user: {}", e);
    } else {
        msg = "Success".to_string();
    }

    web::Json(CommonMessage::new(msg, err))
}

#[derive(MultipartForm)]
struct SponsoredForm {
    id: Text<i64>,
    description: Text<String>,
    active: Text<String>,
    media: Option<TempFile>,
}

#[derive(Deserialize)]
struct SponsoredQuery {
    active: Option<bool>,
}

#[derive(Serialize, Clone)]
struct SponsoredItem {
    id: i64,
    description: String,
    active: bool,
    media: String,
}

#[derive(Serialize)]
struct SponsoredResponse {
    ads: Vec<SponsoredItem>,
    err: String,
}

#[get("")]
async fn sponsored_search(db_pool: web::Data<DBPool>, queries: web::Query<SponsoredQuery>) -> impl Responder {
    let mut ads = Vec::new();
    let mut err = String::new();
    let cond = AdsCond { active: queries.active };
    match db_ads::search(&db_pool, &cond).await {
        Ok(rows) => ads = rows.into_iter().map(sponsored_view).collect(),
        Err(e) => err = e.to_string(),
    }
    web::Json(SponsoredResponse { ads, err })
}

#[get("/{id}")]
async fn sponsored_get(db_pool: web::Data<DBPool>, id: web::Path<i64>) -> impl Responder {
    let mut ads = Vec::new();
    let mut err = String::new();
    match db_ads::get(&db_pool, id.into_inner()).await {
        Ok(row) => ads.push(sponsored_view(row)),
        Err(e) => err = e.to_string(),
    }
    web::Json(SponsoredResponse { ads, err })
}

#[get("/delete/{id}")]
async fn sponsored_delete(
    db_pool: web::Data<DBPool>,
    config_data: web::Data<ConfigData>,
    id: web::Path<i64>,
) -> impl Responder {
    let ads_id = id.into_inner();
    let config = config_data.config.read().await;
    let data_dir = PathBuf::from(&config.data_dir);
    if let Ok(old) = db_ads::get(&db_pool, ads_id).await {
        delete_file(&data_dir, &old.media).await;
    }
    let res = if let Err(e) = db_ads::delete(&db_pool, ads_id).await {
        CommonMessage::from_err(e.to_string())
    } else {
        CommonMessage::from_msg(String::from("Success"))
    };
    web::Json(res)
}

#[post("")]
async fn sponsored_update(
    db_pool: web::Data<DBPool>,
    config_data: web::Data<ConfigData>,
    MultipartForm(data): MultipartForm<SponsoredForm>,
) -> impl Responder {
    let mut err = String::new();
    let mut msg = String::new();
    let config = config_data.config.read().await;
    let data_dir = PathBuf::from(&config.data_dir);
    let description = data.description.0.trim().to_string();
    if description.is_empty() {
        return web::Json(CommonMessage::from_err("Title is required".to_string()));
    }

    let mut ads = Ads {
        id: data.id.0,
        description,
        active: parse_bool(&data.active.0),
        media: String::new(),
    };

    if ads.id > 0 {
        if let Ok(old) = db_ads::get(&db_pool, ads.id).await {
            ads.media = old.media;
        }
        if let Err(e) = db_ads::update(&db_pool, &ads).await {
            err = e.to_string();
            return web::Json(CommonMessage::new(msg, err));
        }
        msg = "Success".to_string();
    } else {
        match db_ads::insert(&db_pool, &ads).await {
            Ok(id) => {
                ads.id = id;
                msg = "Success".to_string();
            }
            Err(e) => {
                err = e.to_string();
                error!("Failed to insert sponsored: {}", e);
                return web::Json(CommonMessage::new(msg, err));
            }
        }
    }

    if let Some(file) = data.media
        && is_real_upload(&file)
    {
        let ext = upload_extension(&file);
        let saved_name = format!("{}.{}", ads.id, ext);
        let rel = format!("ads/{saved_name}");
        if !ads.media.is_empty() && ads.media != rel {
            delete_file(&data_dir, &ads.media).await;
        }
        let ads_dir = data_dir.join("ads");
        match save_file_overwrite(&ads_dir, file, &saved_name).await {
            Ok(_) => {
                ads.media = rel;
                if let Err(e) = db_ads::update(&db_pool, &ads).await {
                    err = e.to_string();
                }
            }
            Err(e) => err = e.to_string(),
        }
    }

    web::Json(CommonMessage::new(msg, err))
}
