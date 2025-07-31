mod api_character;
mod api_post;

use actix_web::web;
use serde::Serialize;

pub fn scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(api_character::scope)
            .configure(api_post::scope),
    );
}

#[derive(Serialize)]
struct CommonMessage {
    msg: String,
    err: String,
}

impl CommonMessage {
    fn new(msg: String, err: String) -> Self {
        Self { msg, err }
    }

    fn from_msg(msg: String) -> Self {
        Self {
            msg,
            err: "".to_string(),
        }
    }

    fn from_err(err: String) -> Self {
        Self {
            msg: "".to_string(),
            err,
        }
    }
}
