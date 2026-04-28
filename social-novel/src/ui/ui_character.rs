use actix_web::{get, web, HttpResponse, Responder};
use tera::Tera;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/character")
            .service(index)
            .service(get)
        ,
    );
}


#[get("")]
async fn index(tmpl: web::Data<Tera>)-> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Characters - Social Novel");
    ctx.insert("active_nav", "character");

    match tmpl.render("characters.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}

#[get("/{id}")]
async fn get(tmpl: web::Data<Tera>)-> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Edit Character - Social Novel");
    ctx.insert("active_nav", "character");

    match tmpl.render("character_edit.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}
