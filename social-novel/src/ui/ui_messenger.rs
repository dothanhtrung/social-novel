use actix_web::{get, web, HttpResponse, Responder};
use tera::Tera;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/messenger")
            .service(index)
        ,
    );
}


#[get("")]
async fn index(tmpl: web::Data<Tera>)-> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("title", "Messenger");
    ctx.insert("active_nav", "messenger");

    match tmpl.render("messenger.html", &ctx) {
        Ok(template) => HttpResponse::Ok().content_type("text/html").body(template),
        Err(e) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Template error: {e}")),
    }
}