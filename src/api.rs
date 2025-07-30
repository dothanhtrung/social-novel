mod character;
mod post;

use actix_web::web;

pub fn scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(character::scope).configure(post::scope));
}
