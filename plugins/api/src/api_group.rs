use std::path::PathBuf;

use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_web::{
    Responder, get, post,
    web::{self, Query},
};
use my_config::ConfigData;
use my_db::{
    DBPool,
    db_group::{self, Group},
    db_post,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{CommonMessage, save_avatar};

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/group").service(search).service(delete).service(update));
}

#[derive(Debug, Deserialize)]
struct GroupQuery {
    #[serde(default)]
    search: String,
}

#[derive(MultipartForm, Debug)]
struct GroupForm {
    id: Text<i64>,
    name: Text<String>,
    description: Text<String>,
    avatar: Option<TempFile>,
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
        Ok(_) => match db_post::delete_by_group(&dbpool, group_id).await {
            Ok(count) => msg = format!("Deleted {count} posts."),
            Err(e) => err = e.to_string(),
        },
        Err(e) => err = e.to_string(),
    }
    // TODO: Delete avatar and post
    web::Json(CommonMessage::new(msg, err))
}

#[post("")]
async fn update(
    dbpool: web::Data<DBPool>,
    config_data: web::Data<ConfigData>,
    MultipartForm(data): MultipartForm<GroupForm>,
) -> impl Responder {
    let mut res = CommonMessage::default();
    let config = config_data.config.read().await;
    debug!("Received form: {:?}", data);

    let mut group = Group {
        id: data.id.into_inner(),
        name: data.name.into_inner(),
        description: data.description.into_inner(),
    };

    if group.id > 0 {
        match db_group::update(&dbpool, &group).await {
            Ok(_) => res.msg = String::from("Success"),
            Err(e) => res.err = e.to_string(),
        }
    } else {
        match db_group::insert(&dbpool, &group).await {
            Ok(id) => group.id = id,
            Err(e) => res.err = e.to_string(),
        }
    }

    if group.id > 0 {
        let file_name = format!("{}.png", group.id);
        let data_dir = PathBuf::from(&config.data_dir);
        if let Some(avatar) = data.avatar {
            save_avatar(&data_dir, avatar, file_name.as_str()).await;
        }
    }

    web::Json(res)
}
