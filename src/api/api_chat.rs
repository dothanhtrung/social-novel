use crate::api::CommonMessage;
use crate::ConfigData;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::web::Query;
use actix_web::{get, post, web, Responder};
use serde::{Deserialize, Serialize};
use sn_internal::db::db_chat::ChatRoom;
use sn_internal::db::{db_chat, DBPool};

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/chat").service(get).service(delete).service(update));
}

#[derive(Serialize)]
struct ChatResponse {
    chats: Vec<ChatRoom>,
    err: Option<String>,
}

#[derive(Deserialize)]
struct ChatQuery {
    id: Option<i64>,
    member: Option<i64>,
}

#[derive(MultipartForm)]
struct ChatForm {
    id: Text<i64>,
    name: Text<String>,
    avatar: Option<TempFile>,
    background: Option<TempFile>,
}

#[get("")]
async fn get(dbpool: web::Data<DBPool>, query_params: Query<ChatQuery>) -> impl Responder {
    let mut chats = Vec::new();
    let mut err = None;

    if let Some(id) = query_params.id.clone() {
        match db_chat::get_by_id(&dbpool, id).await {
            Ok(_ret) => chats = _ret,
            Err(e) => {
                err = Some(e.to_string());
            }
        }
    } else if let Some(member) = query_params.member.clone() {
        match db_chat::get_by_member(&dbpool, member).await {
            Ok(_ret) => chats = _ret,
            Err(e) => {
                err = Some(e.to_string());
            }
        }
    }

    ChatResponse { chats, err }
}

#[post("")]
async fn update(
    dbpool: web::Data<DBPool>,
    query_params: Query<ChatQuery>,
) -> impl Responder {
    let mut chats = Vec::new();
    let mut err = None;

    ChatResponse { chats, err }
}

#[get("delete/{id}")]
async fn delete(dbpool: web::Data<DBPool>, id: web::Path<i64>) -> impl Responder {
    CommonMessage::default()
}

#[get("kick/{id}")]
async fn kick(dbpool: web::Data<DBPool>, member: web::Path<i64>) -> impl Responder {
    CommonMessage::default()
}