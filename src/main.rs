mod db;
mod route;

use std::env;
use actix_files::Files;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use clap::Parser;
use configparser::ini::Ini;
use std::path::{Path, PathBuf};
use dotenvy::dotenv;
use sqlx::postgres::PgPool;
use tera::Tera;
use crate::route::character;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "config.ini")]
    config: String,
}

pub struct AppState {
    pub pool: PgPool,
    pub root_dir: PathBuf,
    thumbnail_dir: PathBuf,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let args = Cli::parse();
    let mut config = Ini::new();
    let _ = config.load(args.config);
    let config_root_dir = config.get("default", "root").unwrap();
    let root_dir = Path::new(&config_root_dir).to_path_buf();
    let thumbnail_dir = root_dir.join("thumbnail");
    let db_path = config.get("default", "db").unwrap();
    let port = config.get("default", "port").unwrap();

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap_or_default())
        .await;
    if pool.is_err() {
        eprintln!("Failed to connect to {db_path}");
        return Ok(());
    }
    let pool = pool.unwrap();

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/res/html/**/*")).unwrap();

        App::new()
            .app_data(Data::new(tera))
            .app_data(Data::new(AppState {
                pool: pool.clone(),
                root_dir: root_dir.clone(),
                thumbnail_dir: thumbnail_dir.clone(),
            }))
            .service(character::users)
            .service(Files::new("/img", root_dir.clone()))
            .service(Files::new(
                "/css",
                concat!(env!("CARGO_MANIFEST_DIR"), "/res/css"),
            ))
            .service(Files::new(
                "/js",
                concat!(env!("CARGO_MANIFEST_DIR"), "/res/js"),
            ))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
