use actix_web::{get, web, Responder};
use serde::Deserialize;
use sn_internal::db::DBPool;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/media").service(get));
}

#[derive(Deserialize)]
struct MediaQuery {
    post: Option<i64>,
    author: Option<i64>,
}

#[get("")]
async fn get(dbpool: web::Data<DBPool>, query: web::Query<MediaQuery>) -> impl Responder {

    web::Json("")
}
