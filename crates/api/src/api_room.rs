use std::path::{Path, PathBuf};

use crate::{delete_files, CommonMessage, save_file};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use my_config::ConfigData;
use my_db::db_media;
use my_db::db_room::{self, RoomMember};
use my_db::db_room::{ChatRoom, RoomQuery};
use serde::{Deserialize, Serialize};
use web_misc::db::{self, DBPool};

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/room")
            .service(get)
            .service(delete)
            .service(update)
            .service(update_llm)
            .service(invite)
            .service(kick),
    );
}

#[derive(Serialize)]
struct RoomResponse {
    rooms: Vec<ChatRoom>,
    err: Option<String>,
}

#[derive(MultipartForm)]
struct RoomForm {
    id: Text<i64>,
    name: Text<String>,
    members: Text<String>,
    background: Option<TempFile>,
}

#[get("")]
async fn get(dbpool: web::Data<DBPool>, query_params: Query<RoomQuery>) -> impl Responder {
    let mut rooms = Vec::new();
    let mut err = None;

    match db_room::search(&dbpool, &query_params).await {
        Ok(_ret) => rooms = _ret,
        Err(e) => {
            err = Some(e.to_string());
        }
    }

    web::Json(RoomResponse { rooms, err })
}

#[post("")]
async fn update(
    config_data: web::Data<ConfigData>,
    dbpool: web::Data<DBPool>,
    MultipartForm(data): MultipartForm<RoomForm>,
) -> impl Responder {
    let mut ret = CommonMessage::default();
    let config = config_data.config.read().await;

    if let Some(background) = data.background {
        let file_name = format!("{}.png", data.id.0);
        let data_dir = PathBuf::from(&config.data_dir).join("background");
        let _ = save_file(&data_dir, background, file_name.as_str()).await;
    }

    let room = ChatRoom {
        id: data.id.0,
        name: data.name.0,
        ..ChatRoom::default()
    };
    let members: Vec<String> = data
        .members
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if room.id > 0 {
        match db_room::update(&dbpool, &room).await {
            Ok(_) => {
                ret.msg = "Success".to_string();
                let _ = db_room::kick_all(&dbpool, room.id).await;
                if let Err(e) = db_room::invite(&dbpool, &RoomMember { room: room.id, members }).await {
                    ret.err = e.to_string();
                }
            }
            Err(e) => ret.err = e.to_string(),
        }
    } else {
        match db_room::insert(&dbpool, &room).await {
            Ok(id) => {
                ret = CommonMessage::from_msg("Success".to_string()).with_id(id);

                if let Err(e) = db_room::invite(&dbpool, &RoomMember { room: id, members }).await {
                    ret.err = e.to_string();
                }
            }
            Err(e) => ret.err = e.to_string(),
        }
    }

    web::Json(ret)
}

#[derive(Deserialize)]
struct RoomLlmBody {
    id: i64,
    llm_enabled: bool,
    llm_recent: i32,
}

fn clamp_llm_recent(n: i32) -> i32 {
    n.clamp(1, 50)
}

#[post("llm")]
async fn update_llm(dbpool: web::Data<DBPool>, body: web::Json<RoomLlmBody>) -> impl Responder {
    if body.id <= 0 {
        return web::Json(CommonMessage::from_err("Room is required".to_string()));
    }
    let room = ChatRoom {
        id: body.id,
        llm_enabled: body.llm_enabled,
        llm_recent: clamp_llm_recent(body.llm_recent),
        ..ChatRoom::default()
    };
    match db_room::update_llm(&dbpool, &room).await {
        Ok(0) => web::Json(CommonMessage::from_err("Room not found".to_string())),
        Ok(_) => web::Json(CommonMessage::from_msg("Success".to_string())),
        Err(e) => web::Json(CommonMessage::from_err(e.to_string())),
    }
}

#[get("delete/{id}")]
async fn delete(
    dbpool: web::Data<DBPool>,
    config_data: web::Data<ConfigData>,
    id: web::Path<i64>,
) -> impl Responder {
    let room_id = id.into_inner();
    let ret = match db_media::unlink_room(&dbpool, room_id).await {
        Ok(urls) => match db_room::delete_by_id(&dbpool, room_id).await {
            Ok(_) => {
                let _ = db_room::kick_all(&dbpool, room_id).await;
                let config = config_data.config.read().await;
                delete_files(Path::new(&config.data_dir), &urls).await;
                CommonMessage::from_msg("Success".to_string())
            }
            Err(e) => CommonMessage::from_err(e.to_string()),
        },
        Err(e) => CommonMessage::from_err(e.to_string()),
    };

    web::Json(ret)
}

#[get("invite")]
async fn invite(dbpool: web::Data<DBPool>, room_member: Query<RoomMember>) -> impl Responder {
    match db_room::invite(&dbpool, &room_member).await {
        Ok(_) => web::Json(CommonMessage::from_msg("Success".to_string())),
        Err(e) => web::Json(CommonMessage::from_err(e.to_string())),
    }
}

#[get("kick")]
async fn kick(dbpool: web::Data<DBPool>, room_member: Query<RoomMember>) -> impl Responder {
    match db_room::kick(&dbpool, &room_member).await {
        Ok(_) => web::Json(CommonMessage::from_msg("Success".to_string())),
        Err(e) => web::Json(CommonMessage::from_err(e.to_string())),
    }
}
