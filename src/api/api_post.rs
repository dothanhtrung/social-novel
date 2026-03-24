use crate::api::{delete_file, save_file, CommonMessage};
use crate::ConfigData;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::web::Query;
use actix_web::{get, post, web, Responder};
use serde::{Deserialize, Serialize};
use sn_internal::db::db_media::Media;
use sn_internal::db::db_post::Post;
use sn_internal::db::{db_media, db_post, DBPool};
use sqlx::types::time::OffsetDateTime;
use std::cmp::max;
use std::path::{Path, PathBuf};

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
}

#[derive(MultipartForm)]
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
    group: Text<i64>,
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
async fn update(
    dbpool: web::Data<DBPool>,
    config_data: web::Data<ConfigData>,
    MultipartForm(data): MultipartForm<PostForm>,
) -> impl Responder {
    let mut msg = String::new();
    let mut err = String::new();
    let config = config_data.config.read().await;
    let form_parent = data.parent.into_inner();
    let parent = if form_parent > 0 { Some(form_parent) } else { None };
    let group = if data.group.0 == 0 { None } else { Some(data.group.0) };
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
        created_at: OffsetDateTime::now_utc(),
        updated_at: OffsetDateTime::now_utc(),
        group,
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

    let data_dir = PathBuf::from(&config.data_dir);
    for media in data.media {
        match save_file(&data_dir, media, "").await {
            Ok((blake3, file_name, file_type)) => {
                let media = Media {
                    id: 0,
                    post: post_id,
                    url: file_name,
                    blake3,
                    file_type,
                };
                if let Err(e) = db_media::insert(&dbpool, &media).await {
                    err = e.to_string();
                }
            }
            Err(e) => {
                if e.to_string() != "exist" {
                    err = e.to_string()
                }
            }
        }
    }

    web::Json(CommonMessage::new(msg, err))
}

#[get("delete/{id}")]
async fn delete(dbpool: web::Data<DBPool>, config_data: web::Data<ConfigData>, id: web::Path<i64>) -> impl Responder {
    let mut msg = String::new();
    let mut err = String::new();
    let config = config_data.config.read().await;
    let post_id = id.into_inner();
    match db_post::delete(&dbpool, post_id).await {
        Ok(_) => {
            if let Ok(urls) = db_media::delete_by_post(&dbpool, post_id).await {
                let data_dir = Path::new(&config.data_dir);
                for url in urls {
                    delete_file(data_dir, url.as_str()).await;
                }
                msg = "Success".to_string();
            }
        }
        Err(e) => err = e.to_string(),
    }
    web::Json(CommonMessage::new(msg, err))
}
