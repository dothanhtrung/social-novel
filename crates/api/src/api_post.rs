use crate::{CommonMessage, delete_files, save_file};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use my_config::ConfigData;
use my_db::db_media::Media;
use my_db::db_post::{Post, SearchPostCondition, string_to_offsetdatetime};
use my_db::{db_media, db_post};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use std::cmp::max;
use std::path::{Path, PathBuf};
use tracing::{debug, error};
use web_misc::db::DBPool;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/post").service(get).service(update).service(delete));
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
    group: Option<i64>, // TODO: query by multiple groups, authors
    room: Option<i64>,
    /// When true, only root posts (parent is null) are returned.
    roots: Option<bool>,
}

#[derive(MultipartForm, Debug)]
struct PostForm {
    id: Text<i64>,
    content: Text<String>,
    media: Vec<TempFile>,
    author: Text<i64>,
    parent: Text<i64>,
    liked: Text<i32>,
    haha: Text<i32>,
    loved: Text<i32>,
    surprised: Text<i32>,
    sad: Text<i32>,
    feeling: Text<String>,
    is_with: Text<String>,
    liked_by: Text<String>,
    group: Text<i64>,
    room: Text<i64>,
    updated_at: Option<Text<String>>,
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
    let page = max(1, query_params.page.unwrap_or(1)) - 1;
    let limit = max(0, query_params.count.unwrap_or(config.api.per_page as i64));
    let offset = page * limit;
    let mut cond = SearchPostCondition::default();
    cond.authors = if query_params.author.is_some() {
        Some(vec![query_params.author.unwrap()])
    } else {
        None
    };
    cond.groups = if query_params.group.is_some() {
        Some(vec![query_params.group.unwrap()])
    } else {
        None
    };
    cond.rooms = if query_params.room.is_some() {
        Some(vec![query_params.room.unwrap()])
    } else {
        None
    };
    cond.roots_only = query_params.roots.unwrap_or(false);

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
        match db_post::get_all(&dbpool, limit, offset, &cond).await {
            Ok(posts) => ret = posts,
            Err(e) => err = e.to_string(),
        }
    }

    web::Json(PostResponse { posts: ret, err })
}

/// Walk parents until a post that belongs to a group is found.
async fn inherit_group(dbpool: &DBPool, mut parent: Option<i64>) -> Option<i64> {
    for _ in 0..32 {
        let Some(parent_id) = parent.filter(|id| *id > 0) else {
            return None;
        };
        let Ok(post) = db_post::get_by_id(dbpool, parent_id).await else {
            return None;
        };
        if let Some(group_id) = post.group.filter(|id| *id > 0) {
            return Some(group_id);
        }
        parent = post.parent;
    }
    None
}

#[post("")]
async fn update(
    dbpool: web::Data<DBPool>,
    config_data: web::Data<ConfigData>,
    MultipartForm(data): MultipartForm<PostForm>,
) -> impl Responder {
    let mut msg = String::new();
    let mut err = String::new();
    let config = config_data.config.read().await;
    debug!("Received form: {:?}", data);
    let form_parent = data.parent.into_inner();
    let parent = if form_parent > 0 { Some(form_parent) } else { None };
    let is_new = data.id.0 == 0;
    let mut group = if data.group.0 == 0 { None } else { Some(data.group.0) };
    // A comment with no group inherits the group of the post (or ancestor) it replies to.
    if is_new && group.is_none() {
        group = inherit_group(&dbpool, parent).await;
    }
    let room = if data.room.0 == 0 { None } else { Some(data.room.0) };
    let updated_at = if let Some(timestamp) = data.updated_at {
        string_to_offsetdatetime(timestamp.into_inner().as_str()).unwrap_or(OffsetDateTime::now_utc())
    } else {
        OffsetDateTime::now_utc()
    };
    let post = Post {
        id: data.id.into_inner(),
        content: data.content.into_inner(),
        author: data.author.into_inner(),
        parent,
        liked: data.liked.into_inner(),
        haha: data.haha.into_inner(),
        loved: data.loved.into_inner(),
        surprised: data.surprised.into_inner(),
        sad: data.sad.into_inner(),
        feeling: data.feeling.into_inner(),
        is_with: data.is_with.into_inner(),
        liked_by: data.liked_by.into_inner(),
        created_at: OffsetDateTime::now_utc(),
        updated_at,
        group,
        room,
    };

    let mut post_id = post.id;
    if post.id > 0 {
        match db_post::update(&dbpool, &post).await {
            Ok(_) => msg = "Success".to_string(),
            Err(e) => {
                err = e.to_string();
                return web::Json(CommonMessage::new(msg, err));
            }
        }
    } else {
        match db_post::insert(&dbpool, &post).await {
            Ok(id) => {
                post_id = id;
                msg = "Success".to_string();
            }
            Err(e) => {
                err = e.to_string();
                return web::Json(CommonMessage::new(msg, err));
            }
        }
    }

    if let Some(room) = room
        && let Err(e) = my_db::db_room::touch(&dbpool, room).await
    {
        error!("Failed to touch room {}: {}", room, e);
    }

    let data_dir = PathBuf::from(&config.data_dir);
    for media in data.media {
        match save_file(&data_dir, media, "").await {
            Ok((blake3, file_name, file_type)) => {
                let media = Media {
                    id: 0,
                    url: file_name,
                    blake3,
                    file_type,
                };
                if let Err(e) = db_media::attach(&dbpool, post_id, &media).await {
                    err = e.to_string();
                }
            }
            Err(e) => err = e.to_string(),
        }
    }

    // id lets the page insert the new comment without reloading.
    web::Json(CommonMessage::new(msg, err).with_id(post_id))
}

#[get("delete/{id}")]
async fn delete(dbpool: web::Data<DBPool>, config_data: web::Data<ConfigData>, id: web::Path<i64>) -> impl Responder {
    let mut msg = String::new();
    let mut err = String::new();
    let config = config_data.config.read().await;
    let post_id = id.into_inner();

    // Drop this post's links. Files still used by another post are kept.
    match db_media::unlink_posts(&dbpool, &[post_id]).await {
        Ok(urls) => {
            match db_post::delete_by_id(&dbpool, post_id).await {
                Ok(_) => {
                    msg = "Success".to_string();
                    delete_files(Path::new(&config.data_dir), &urls).await;
                }
                Err(e) => err = e.to_string(),
            }
        }
        Err(e) => err = e.to_string(),
    }
    web::Json(CommonMessage::new(msg, err))
}
