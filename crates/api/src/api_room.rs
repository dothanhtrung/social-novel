use std::path::PathBuf;

use crate::{CommonMessage, save_file};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use my_config::ConfigData;
use my_db::db_room::{self, RoomMember};
use my_db::db_room::{ChatRoom, RoomQuery};
use serde::Serialize;
use web_misc::db::{self, DBPool};

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/room")
            .service(get)
            .service(delete)
            .service(update)
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
                ret.msg = "Success".to_string();

                if let Err(e) = db_room::invite(&dbpool, &RoomMember { room: id, members }).await {
                    ret.err = e.to_string();
                }
            }
            Err(e) => ret.err = e.to_string(),
        }
    }

    web::Json(ret)
}

#[get("delete/{id}")]
async fn delete(dbpool: web::Data<DBPool>, id: web::Path<i64>) -> impl Responder {
    let room_id = id.into_inner();
    let ret = match db_room::delete_by_id(&dbpool, room_id).await {
        Ok(_) => {
            let _ = db_room::kick_all(&dbpool, room_id).await;
            CommonMessage::from_msg("Success".to_string())
        
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
