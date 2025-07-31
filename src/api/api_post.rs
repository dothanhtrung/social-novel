use crate::api::CommonMessage;
use crate::ConfigData;
use actix_web::web::Query;
use actix_web::{get, post, web, Responder};
use serde::{Deserialize, Serialize};
use sn_internal::db::db_post::Post;
use sn_internal::db::{db_post, DBPool};
use std::cmp::max;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/post").service(get).service(update));
}

#[derive(Serialize)]
struct PostResponse {
    posts: Vec<Post>,
    err: String,
}

#[derive(Deserialize)]
struct PostQuery {
    id: Option<i64>,
    page: Option<i64>,
    count: Option<i64>,
    author: Option<i64>,
    parent: Option<i64>,
}

#[get("")]
async fn get(
    config_data: web::Data<ConfigData>,
    dbpool: web::Data<DBPool>,
    query_params: Query<PostQuery>,
) -> impl Responder {
    let config = config_data.config.read().await;
    let mut ret = Vec::new();
    let mut err = String::new();
    if let Some(id) = query_params.id {
        match db_post::get_by_id(&dbpool, id).await {
            Ok(post) => ret = vec![post],
            Err(e) => err = e.to_string(),
        }
    } else if let Some(parent) = query_params.parent {
        match db_post::get_by_parent(&dbpool, parent).await {
            Ok(posts) => ret = posts,
            Err(e) => err = e.to_string(),
        }
    } else {
        let page = max(1, query_params.page.unwrap_or(1)) - 1;
        let limit = max(0, query_params.count.unwrap_or(config.api.per_page as i64));
        let offset = page * limit;
        if let Some(author) = query_params.author {
            match db_post::get_by_author(&dbpool, author, limit, offset).await {
                Ok(posts) => ret = posts,
                Err(e) => err = e.to_string(),
            }
        } else {
            match db_post::get_all(&dbpool, limit, offset).await {
                Ok(posts) => ret = posts,
                Err(e) => err = e.to_string(),
            }
        }
    }

    web::Json(PostResponse { posts: ret, err })
}

#[post("")]
async fn update(dbpool: web::Data<DBPool>, post: web::Json<Post>) -> impl Responder {
    let mut msg = String::new();
    let mut err = String::new();

    if post.id > 0 {
        match db_post::update(&dbpool, &post).await {
            Ok(_) => msg = "Success".to_string(),
            Err(e) => err = e.to_string(),
        }
    } else {
        match db_post::insert(&dbpool, &post).await {
            Ok(_) => msg = "Success".to_string(),
            Err(e) => err = e.to_string(),
        }
    }

    web::Json(CommonMessage::new(msg, err))
}

#[get("delete/{id}")]
async fn delete(dbpool: web::Data<DBPool>, id: web::Path<i64>) -> impl Responder {
    let mut msg = String::new();
    let mut err = String::new();
    match db_post::delete(&dbpool, id.into_inner()).await {
        Ok(_) => msg = "Success".to_string(),
        Err(e) => err = e.to_string(),
    }
    web::Json(CommonMessage::new(msg, err))
}
