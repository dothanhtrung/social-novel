use actix_web::{get, web, HttpResponse, Responder};
use tera::Tera;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(profile)
        .service(group_page)
        .service(get)
        .service(edit);
}

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Social Novel");
    ctx.insert("active_nav", "home");

    match tmpl.render("index.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}

#[get("/g/{id}")]
async fn group_page(tmpl: web::Data<Tera>, id: web::Path<i64>) -> impl Responder {
    let group_id = id.into_inner();
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Group - Social Novel");
    ctx.insert("active_nav", "groups");
    ctx.insert("group_id", &group_id);

    match tmpl.render("index.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}

#[get("/u/{name}")]
async fn profile(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Profile - Social Novel");
    match tmpl.render("profile.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}

#[get("/p/{id}")]
async fn get(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Post - Social Novel");
    ctx.insert("active_nav", "home");

    match tmpl.render("post.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}

#[get("/p/edit/{id}")]
async fn edit(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Edit Post - Social Novel");
    ctx.insert("active_nav", "home");
    match tmpl.render("post_edit.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}
