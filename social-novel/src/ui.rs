mod ui_character;
mod ui_post;
mod ui_messenger;

use actix_files::Files;
use actix_web::web;
use actix_web::web::Data;
use tera::Tera;
use web_misc::web::broadcaster::event_stream;

pub fn scope_config(cfg: &mut web::ServiceConfig) {
    let tera = Tera::new("res/html/**/*").unwrap();

    cfg.app_data(Data::new(tera))
        .service(event_stream)
        .service(Files::new("/assets", "res/assets"))
        .service(Files::new("/css", "res/css"))
        .service(Files::new("/js", "res/js"))
        .configure(ui_character::scope)
        .configure(ui_messenger::scope)
        .configure(ui_post::scope);
}

macro_rules! redirect {
    ($url: expr) => {
        HttpResponse::Found().append_header(("Location", $url)).finish()
    };
}
