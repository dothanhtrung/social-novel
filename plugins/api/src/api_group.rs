
use actix_web::{
    Responder, get,
    web::{self, Query},
};
use serde::{Deserialize, Serialize};
use my_db::{
    DBPool,
    db_group::{self, Group}, db_post,
};

use crate::CommonMessage;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/group").service(search));
}

#[derive(Debug, Deserialize)]
struct GroupQuery {
    search: String,
}

#[derive(Serialize, Default)]
struct GroupResponse {
    groups: Vec<Group>,
    err: String,
}

#[get("")]
async fn search(dbpool: web::Data<DBPool>, query_params: Query<GroupQuery>) -> impl Responder {
    let mut res = GroupResponse::default();
    match db_group::search(&dbpool, &query_params.search).await {
        Ok(ret) => res.groups = ret,
        Err(e) => res.err = e.to_string(),
    }

    web::Json(res)
}

#[get("delete/{id}")]
async fn delete(dbpool: web::Data<DBPool>, id: web::Path<i64>) -> impl Responder {
    let mut msg = String::new();
    let mut err = String::new();
    let group_id = id.into_inner();
    match db_group::delete_by_id(&dbpool, group_id).await {
        Ok(_) => {
            match db_post::delete_by_group(&dbpool, group_id).await {
                Ok(count) => msg = format!("Deleted {count} posts."),
                Err(e) => err = e.to_string(),
            }
        }
        Err(e) => err = e.to_string(),
    }
    web::Json(CommonMessage::new(msg, err))
}
