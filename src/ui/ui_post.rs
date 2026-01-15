use actix_web::{get, web, HttpResponse, Responder};
use tera::Tera;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(get).service(edit);
}

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    // ctx.insert("search", &query_params.search.clone().unwrap_or_default());

    match tmpl.render("index.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}

#[get("/{id}")]
async fn get(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    // ctx.insert("search", &query_params.search.clone().unwrap_or_default());

    match tmpl.render("post.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}

#[get("/post/edit/{id}")]
async fn edit(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    match tmpl.render("post_edit.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}

//
// #[derive(MultipartForm)]
// struct PostForm {
//     content: Text<String>,
//     character: Text<String>,
//     media: Option<TempFile>,
//     reaction: Option<Text<i32>>,
//     parent: Option<Text<i32>>,
//     feeling: Option<Text<String>>,
//     is_with: Option<Text<String>>,
// }
//
// #[derive(Serialize)]
// struct CtxPost {
//     pub id: i32,
//     pub comments: Vec<CtxPost>,
//     pub content: String,
//     pub username: String,
//     pub name: String,
//     pub reaction: i32,
//     pub media: String,
//     pub created_at: String,
//     pub feeling: String,
//     pub is_with: String,
// }
//
// impl CtxPost {
//     fn from_db_post(post: Post) -> Self {
//         let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
//         let created_at = post.created_at.format(&format).unwrap();
//         Self {
//             id: post.id,
//             comments: Vec::new(),
//             content: post.content,
//             username: post.character,
//             name: post.character_name,
//             reaction: post.reaction,
//             media: post.media.unwrap_or_default(),
//             created_at,
//             feeling: post.feeling,
//             is_with: post.is_with,
//         }
//     }
// }
//
// #[derive(Deserialize)]
// struct QueryInfo {
//     page: Option<i64>,
// }
//
// #[get("/")]
// pub async fn index(
//     data: web::Data<AppState>,
//     tmpl: web::Data<tera::Tera>,
//     query: web::Query<QueryInfo>,
// ) -> impl Responder {
//     let mut ctx = tera::Context::new();
//     ctx.insert("subtitle", "");
//
//     let page = query.page.unwrap_or(1);
//     ctx.insert("page", &page);
//
//     let mut posts = Vec::new();
//     if let Ok(_posts) = db::post::get_by_page(&data.pool, data.ipp, page).await {
//         for _post in _posts {
//             let mut post = CtxPost::from_db_post(_post);
//             if let Ok(comments) = db::post::get_by_parent(&data.pool, post.id).await {
//                 for _comment in comments {
//                     let mut comment = CtxPost::from_db_post(_comment);
//
//                     if let Ok(comments2) = db::post::get_by_parent(&data.pool, comment.id).await {
//                         for _comment2 in comments2 {
//                             comment.comments.push(CtxPost::from_db_post(_comment2));
//                         }
//                     }
//
//                     post.comments.push(comment);
//                 }
//             }
//             posts.push(post);
//         }
//
//         ctx.insert("posts", &posts);
//     }
//
//     let template = tmpl.render("index.html", &ctx).unwrap_or("Not found".into());
//     HttpResponse::Ok().content_type("text/html").body(template)
// }
//
// #[post("/")]
// pub async fn add_post(data: web::Data<AppState>, MultipartForm(form): MultipartForm<PostForm>) -> impl Responder {
//     let parent = if let Some(_p) = form.parent { Some(_p.0) } else { None };
//
//     let post_dir = &data.root_dir.join("post");
//     if let Ok((md5sum, filename)) = save_file(&post_dir, form.media, "") {
//         if !md5sum.is_empty() && !filename.is_empty() {
//             let path_file = format!("post/{}", filename);
//             if let Err(e) = db::media::add(&data.pool, md5sum.as_str(), path_file.as_str()).await {
//                 log::error!("Failed to add media: {}", e);
//             }
//         }
//
//         let reaction = if let Some(r) = form.reaction { r.0 } else { 0 };
//         let is_with = if let Some(w) = form.is_with { w.0 } else { String::new() };
//         let feeling = if let Some(f) = form.feeling {f.0} else {String::new()};
//         if let Err(e) = db::post::add(&data.pool, parent, form.content.0, form.character.0, md5sum, reaction, is_with, feeling).await {
//             log::error!("Failed to add post: {}", e);
//             return redirect!("/");
//         }
//     }
//
//     redirect!("/")
// }
//
// #[get("/delete/post/{id}")]
// pub async fn delete_post(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
//     let id = id.into_inner();
//     let md5sum = db::post::get_media(&data.pool, id).await.unwrap_or_default();
//     if let Err(e) = db::post::delete(&data.pool, id).await {
//         log::error!("Failed to delete post {}: {}", id, e);
//         return redirect!("/");
//     }
//
//     if !md5sum.is_empty() {
//         if let Ok(file_path) = db::media::delete(&data.pool, md5sum.as_str()).await {
//             delete_file(&data.root_dir, file_path);
//         }
//     }
//
//     redirect!("/")
// }
