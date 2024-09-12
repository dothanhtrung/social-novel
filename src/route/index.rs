use std::path::PathBuf;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::cookie::time::{format_description, PrimitiveDateTime};
use serde::{Deserialize, Serialize};
use crate::{db, AppState};
use crate::db::post::Post;
use crate::route::{redirect, save_file};

#[derive(MultipartForm)]
struct PostForm {
    content: Text<String>,
    character: Text<String>,
    media: Option<TempFile>,
    reaction: Text<i32>,
    parent: Option<Text<i32>>,
    feeling: Option<Text<String>>,
    is_with: Option<Text<String>>,
}

#[derive(Serialize)]
struct CtxPost {
    pub id: i32,
    pub parent: Option<i32>,
    pub content: String,
    pub username: String,
    pub name: String,
    pub reaction: i32,
    pub media: String,
    pub created_at: String,
    pub feeling: String,
    pub is_with: String,
}

impl CtxPost {
    fn from_db_post(post: Post) -> Self {
        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        let created_at = post.created_at.format(&format).unwrap();
        Self {
            id: post.id,
            parent: post.parent,
            content: post.content,
            username: post.character,
            name: post.character_name,
            reaction: post.reaction,
            media: post.media,
            created_at,
            feeling: post.feeling,
            is_with: post.is_with,
        }
    }
}

#[derive(Deserialize)]
struct QueryInfo {
    page: Option<i64>,
}

#[get("/")]
pub async fn index(data: web::Data<AppState>, tmpl: web::Data<tera::Tera>, query: web::Query<QueryInfo>) -> impl Responder {
    let mut ctx = tera::Context::new();

    let page = query.page.unwrap_or(1);
    let mut posts = vec![];
    if let Ok(_posts) = db::post::get_by_page(&data.pool, data.ipp, page).await {
        for post in _posts {
            posts.push(CtxPost::from_db_post(post));
        }
        ctx.insert("posts", &posts);
    }

    let template = tmpl.render("index.html", &ctx).unwrap_or("Not found".into());
    HttpResponse::Ok().content_type("text/html").body(template)
}

#[post("/")]
pub async fn add_post(data: web::Data<AppState>, MultipartForm(form): MultipartForm<PostForm>) -> impl Responder {
    let parent = if let Some(_p) = form.parent {
        Some(_p.0)
    } else { None };

    if let Ok(media) = save_file(&data.root_dir, form.media, "") {
        if let Err(e) = db::post::add(&data.pool, parent, form.content.0, form.character.0, media, form.reaction.0).await {
            log::error!("Failed to add post: {}", e);
            return redirect!("/");
        }
    }

    redirect!("/")
}

#[get("/delete/post/{id}")]
pub async fn delete_post(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();
    if let Err(e) = db::post::delete(&data.pool, id).await {
        log::error!("Failed to delete post {}: {}", id, e);
    }
    redirect!("/characters")
}

