use actix_web::{Responder, get, post, web};
use my_db::db_friend::{self, Friend};
use serde::{Deserialize, Serialize};
use web_misc::db::DBPool;

use crate::CommonMessage;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/friend")
            .service(list)
            .service(remove)
            .service(add),
    );
}

#[derive(Deserialize)]
struct FriendQuery {
    character: Option<i64>,
    friend: Option<i64>,
}

#[derive(Deserialize)]
struct FriendForm {
    character: i64,
    friend: i64,
}

#[derive(Serialize)]
struct FriendResponse {
    friends: Vec<Friend>,
    err: String,
}

#[get("")]
async fn list(dbpool: web::Data<DBPool>, query: web::Query<FriendQuery>) -> impl Responder {
    let Some(character_id) = query.character.filter(|id| *id > 0) else {
        return web::Json(FriendResponse {
            friends: Vec::new(),
            err: "Character is required".to_string(),
        });
    };
    match db_friend::list(&dbpool, character_id).await {
        Ok(friends) => web::Json(FriendResponse {
            friends,
            err: String::new(),
        }),
        Err(e) => web::Json(FriendResponse {
            friends: Vec::new(),
            err: e.to_string(),
        }),
    }
}

#[post("")]
async fn add(dbpool: web::Data<DBPool>, body: web::Json<FriendForm>) -> impl Responder {
    if body.character <= 0 || body.friend <= 0 {
        return web::Json(CommonMessage::from_err("Both characters are required".to_string()));
    }
    if body.character == body.friend {
        return web::Json(CommonMessage::from_err(
            "A character cannot be friends with themselves".to_string(),
        ));
    }
    match db_friend::insert(&dbpool, body.character, body.friend).await {
        Ok(_) => web::Json(CommonMessage::from_msg("Success".to_string())),
        Err(e) => {
            if is_unique_violation(&e) {
                web::Json(CommonMessage::from_msg("Already friends".to_string()))
            } else {
                web::Json(CommonMessage::from_err(e.to_string()))
            }
        }
    }
}

#[get("delete")]
async fn remove(dbpool: web::Data<DBPool>, query: web::Query<FriendQuery>) -> impl Responder {
    let (Some(character_id), Some(friend_id)) = (
        query.character.filter(|id| *id > 0),
        query.friend.filter(|id| *id > 0),
    ) else {
        return web::Json(CommonMessage::from_err("Both characters are required".to_string()));
    };
    match db_friend::delete_pair(&dbpool, character_id, friend_id).await {
        Ok(_) => web::Json(CommonMessage::from_msg("Success".to_string())),
        Err(e) => web::Json(CommonMessage::from_err(e.to_string())),
    }
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505"))
}
