use std::path::{Path, PathBuf};

use crate::{delete_file, delete_files, save_avatar, save_cover, CommonMessage};
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{
    get, post, web::{self, Query},
    Responder,
};
use my_config::ConfigData;
use my_db::{
    db_group::{self, Group},
    db_media, db_post,
};
use serde::{Deserialize, Serialize};
use tracing::debug;
use web_misc::db::DBPool;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/group")
            .service(members)
            .service(join)
            .service(leave)
            .service(search)
            .service(delete)
            .service(update),
    );
}

#[derive(Debug, Deserialize)]
struct GroupQuery {
    #[serde(default)]
    search: String,
    member: Option<i64>,
    character: Option<i64>,
    group: Option<i64>,
}

#[derive(MultipartForm, Debug)]
struct GroupForm {
    id: Text<i64>,
    name: Text<String>,
    description: Text<String>,
    avatar: Option<TempFile>,
    cover: Option<TempFile>,
}

#[derive(Serialize, Default)]
struct GroupResponse {
    groups: Vec<Group>,
    err: String,
}

#[get("")]
async fn search(dbpool: web::Data<DBPool>, query_params: Query<GroupQuery>) -> impl Responder {
    let mut res = GroupResponse::default();
    let member = query_params.member.filter(|id| *id > 0);
    match db_group::search(&dbpool, &query_params.search, member).await {
        Ok(ret) => res.groups = ret,
        Err(e) => res.err = e.to_string(),
    }

    web::Json(res)
}

#[derive(Serialize)]
struct MemberResponse {
    members: Vec<my_db::db_group::GroupMember>,
    err: String,
}

#[get("members")]
async fn members(dbpool: web::Data<DBPool>, query: Query<GroupQuery>) -> impl Responder {
    let Some(group_id) = query.group.filter(|id| *id > 0) else {
        return web::Json(MemberResponse {
            members: Vec::new(),
            err: "Group is required".to_string(),
        });
    };
    match db_group::members(&dbpool, group_id).await {
        Ok(members) => web::Json(MemberResponse {
            members,
            err: String::new(),
        }),
        Err(e) => web::Json(MemberResponse {
            members: Vec::new(),
            err: e.to_string(),
        }),
    }
}

fn membership_ids(query: &GroupQuery) -> Result<(i64, i64), String> {
    match (query.character.filter(|id| *id > 0), query.group.filter(|id| *id > 0)) {
        (Some(character_id), Some(group_id)) => Ok((character_id, group_id)),
        _ => Err("Character and group are required".to_string()),
    }
}

#[post("join")]
async fn join(dbpool: web::Data<DBPool>, query: Query<GroupQuery>) -> impl Responder {
    let (character_id, group_id) = match membership_ids(&query) {
        Ok(ids) => ids,
        Err(e) => return web::Json(CommonMessage::from_err(e)),
    };
    match db_group::join(&dbpool, character_id, group_id).await {
        Ok(_) => web::Json(CommonMessage::from_msg("Success".to_string())),
        Err(e) => web::Json(CommonMessage::from_err(e.to_string())),
    }
}

#[get("leave")]
async fn leave(dbpool: web::Data<DBPool>, query: Query<GroupQuery>) -> impl Responder {
    let (character_id, group_id) = match membership_ids(&query) {
        Ok(ids) => ids,
        Err(e) => return web::Json(CommonMessage::from_err(e)),
    };
    match db_group::leave(&dbpool, character_id, group_id).await {
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
    let mut msg = String::new();
    let mut err = String::new();
    let group_id = id.into_inner();
    // Unlink before the group delete cascades posts, so shared media is kept.
    match db_media::unlink_group(&dbpool, group_id).await {
        Ok(urls) => {
            match db_group::delete_by_id(&dbpool, group_id).await {
                Ok(_) => match db_post::delete_by_group(&dbpool, group_id).await {
                    Ok(count) => {
                        msg = format!("Deleted {count} posts.");
                        let config = config_data.config.read().await;
                        delete_files(Path::new(&config.data_dir), &urls).await;
                    }
                    Err(e) => err = e.to_string(),
                },
                Err(e) => err = e.to_string(),
            }
        }
        Err(e) => err = e.to_string(),
    }
    // TODO: Delete avatar
    if msg.starts_with("Deleted") {
        let config = config_data.config.read().await;
        delete_file(Path::new(&config.data_dir), &format!("cover/{group_id}.png")).await;
    }
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
            Ok(id) => {
                group.id = id;
                res = CommonMessage::from_msg("Success".to_string()).with_id(id);
            }
            Err(e) => res.err = e.to_string(),
        }
    }

    if group.id > 0 {
        let file_name = format!("{}.png", group.id);
        let data_dir = PathBuf::from(&config.data_dir);
        if let Some(avatar) = data.avatar.filter(|file| file.size > 0) {
            save_avatar(&data_dir, avatar, file_name.as_str()).await;
        }
        if let Some(cover) = data.cover.filter(|file| file.size > 0) {
            save_cover(&data_dir, cover, file_name.as_str()).await;
        }
    }

    web::Json(res)
}
