use actix_web::{get, web, Responder};
use my_db::db_media::Media;
use my_db::{db_media};
use serde::{Deserialize, Serialize};
use web_misc::db::DBPool;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/media").service(get));
}

#[derive(Deserialize)]
struct MediaQuery {
    post: Option<i64>,
    author: Option<i64>,
}

#[derive(Serialize, Default)]
struct MediaResponse {
    media: Vec<Media>,
    err: String,
}

#[get("")]
async fn get(dbpool: web::Data<DBPool>, query: web::Query<MediaQuery>) -> impl Responder {
    let mut res = MediaResponse::default();
    if let Some(post_id) = query.post {
        match db_media::get_by_post(&dbpool, post_id).await {
            Ok(media) => res.media = media,
            Err(e) => res.err = e.to_string(),
        }
    } else if let Some(author) = query.author {
        // TODO
    }
    web::Json(res)
}
