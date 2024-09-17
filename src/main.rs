mod db;
mod route;

use actix_files::Files;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use clap::Parser;
use configparser::ini::Ini;
use dotenvy::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::path::{Path, PathBuf};
use tera::Tera;

use crate::route::character;
use crate::route::index;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "config.ini")]
    config: String,
}

pub struct AppState {
    pub pool: PgPool,
    pub root_dir: PathBuf,
    pub thumbnail_dir: PathBuf,
    pub ipp: i64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let args = Cli::parse();
    let mut config = Ini::new();

    let _ = config.load(args.config);
    let config_root_dir = config.get("default", "root").unwrap();
    let root_dir = Path::new(&config_root_dir).to_path_buf();
    let thumbnail_dir = root_dir.join("thumbnail");
    let db_path = config.get("default", "db").unwrap_or("".to_string());
    let port = config.get("default", "port").unwrap_or("10000".to_string());
    let ipp: i64 = config
        .get("default", "ipp")
        .unwrap_or("24".to_string())
        .parse()
        .unwrap_or(24);

    let pool = PgPool::connect(&db_path).await;
    if pool.is_err() {
        log::error!("Failed to connect to {db_path}");
        return Ok(());
    }
    let pool = pool.unwrap();

    println!("Server will start at :{port}");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/res/html/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .app_data(Data::new(AppState {
                pool: pool.clone(),
                root_dir: root_dir.clone(),
                thumbnail_dir: thumbnail_dir.clone(),
                ipp,
            }))
            .service(index::index)
            .service(index::add_post)
            .service(index::delete_post)
            .service(character::characters)
            .service(character::character)
            .service(character::add_character)
            .service(character::update_character)
            .service(character::delete_character)
            .service(Files::new("/img", root_dir.clone()))
            .service(Files::new("/css", concat!(env!("CARGO_MANIFEST_DIR"), "/res/css")))
            .service(Files::new("/js", concat!(env!("CARGO_MANIFEST_DIR"), "/res/js")))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
