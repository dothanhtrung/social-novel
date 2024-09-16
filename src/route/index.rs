use crate::db::post::Post;
use crate::route::{redirect, save_file};
use crate::{db, AppState};
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::cookie::time::{format_description, PrimitiveDateTime};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use actix_web::web::post;

#[derive(MultipartForm)]
struct PostForm {
    content: Text<String>,
    character: Text<String>,
    media: Option<TempFile>,
    reaction: Option<Text<i32>>,
    parent: Option<Text<i32>>,
    feeling: Option<Text<String>>,
    is_with: Option<Text<String>>,
}

#[derive(Serialize)]
struct CtxPost {
    pub id: i32,
    pub comments: Vec<CtxPost>,
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
            comments: Vec::new(),
            content: post.content,
            username: post.character,
            name: post.character_name,
            reaction: post.reaction,
            media: post.media.unwrap_or_default(),
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
pub async fn index(
    data: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
    query: web::Query<QueryInfo>,
) -> impl Responder {
    let mut ctx = tera::Context::new();

    let page = query.page.unwrap_or(1);
    let mut posts = Vec::new();
    if let Ok(_posts) = db::post::get_by_page(&data.pool, data.ipp, page).await {
        for _post in _posts {
            let mut post = CtxPost::from_db_post(_post);
            if let Ok(comments) = db::post::get_by_parent(&data.pool, post.id).await {
                for _comment in comments {
                    let mut comment = CtxPost::from_db_post(_comment);

                    if let Ok(comments2) = db::post::get_by_parent(&data.pool, comment.id).await {
                        for _comment2 in comments2 {
                            comment.comments.push(CtxPost::from_db_post(_comment2));
                        }
                    }

                    post.comments.push(comment);
                }
            }
            posts.push(post);
        }

        ctx.insert("posts", &posts);
    }

    let template = tmpl.render("index.gohtml", &ctx).unwrap_or("Not found".into());
    HttpResponse::Ok().content_type("text/html").body(template)
}

#[post("/")]
pub async fn add_post(data: web::Data<AppState>, MultipartForm(form): MultipartForm<PostForm>) -> impl Responder {
    let parent = if let Some(_p) = form.parent { Some(_p.0) } else { None };

    let post_dir = &data.root_dir.join("post");
    if let Ok((md5sum, filename)) = save_file(&post_dir, form.media, "") {
        let path_file = post_dir.join(filename);
        if let Err(e) = db::media::add(&data.pool, md5sum.as_str(), path_file.to_str().unwrap_or_default()).await {
            log::error!("Failed to add media: {}", e);
            return redirect!("/");
        }

        let reaction = if let Some(r) = form.reaction { r.0 } else { 0 };
        if let Err(e) = db::post::add(&data.pool, parent, form.content.0, form.character.0, md5sum, reaction).await {
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
    redirect!("/")
}
