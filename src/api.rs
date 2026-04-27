mod api_character;
mod api_chat;
mod api_group;
mod api_media;
mod api_post;

use actix_multipart::form::tempfile::TempFile;
use actix_web::http::StatusCode;
use actix_web::{ResponseError, web};
use anyhow::anyhow;
use core::fmt::Formatter;
use serde::{Deserialize, Serialize};
use sn_internal::db::db_media::MediaType;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{error, info, warn};

pub fn scope_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(api_character::scope)
            .configure(api_group::scope)
            .configure(api_media::scope)
            .configure(api_post::scope),
    );
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ErrorResponse {
    MethodNotAllowed(String),
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
}
impl Display for ErrorResponse {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        todo!()
    }
}

#[derive(Serialize, Default)]
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

async fn save_file(
    target_dir: &PathBuf,
    temp_file: TempFile,
    file_name: &str,
) -> Result<(String, String, MediaType), anyhow::Error> {
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).await?;
    }

    let mut hasher = blake3::Hasher::new();
    let temp_file_name = temp_file.file_name.unwrap_or_default();
    let extension = Path::new(temp_file_name.as_str())
        .extension()
        .unwrap_or(OsStr::new("png"))
        .to_str()
        .unwrap_or("png");
    let open_f = temp_file.file.reopen()?;
    let mut reader = BufReader::new(open_f);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    hasher.update(&buf);
    let blake3_hash = hasher.finalize().to_hex().to_string().to_lowercase();

    let saved_name = if file_name.is_empty() {
        format!("{}.{}", blake3_hash, extension)
    } else {
        String::from(file_name)
    };

    let path = target_dir.join(saved_name.as_str());
    if !path.exists() {
        if let Err(e) = temp_file.file.persist(&path) {
            warn!("Failed to save file to {:?}: {}", &path.to_str(), e);
            warn!("Try copying");
            fs::copy(e.file.path(), &path).await?;
        }
    } else {
        info!("File {saved_name} exists");
        return Err(anyhow!("exist"));
    }

    let file_type = file_type(&path).await;

    Ok((blake3_hash, saved_name, file_type))
}

async fn delete_file(root_dir: &Path, file_path: &str) {
    if !file_path.is_empty() {
        let real_path = root_dir.join(&file_path);
        if let Err(e) = fs::remove_file(&real_path).await {
            error!("Failed to remove file {}: {}", file_path, e);
        }
    }
}

async fn save_avatar(root_dir: &Path, avatar: TempFile, file_name: &str) {
    let path = root_dir.join("avatar");
    if save_file(&path, avatar, file_name).await.is_ok() {
        info!("Saved avatar successfully");
    }
}

fn calculate_blake3(file_path: &Path) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(result.to_hex().to_string().to_lowercase())
}

async fn file_type(path: &Path) -> MediaType {
    let data = fs::read(path).await.ok().unwrap_or_default();
    if let Some(kind) = infer::get(&data) {
        if kind.mime_type().starts_with("video/") {
            return MediaType::Video;
        } else if kind.mime_type().starts_with("image/") {
            return MediaType::Image;
        }
    }

    MediaType::NA
}
