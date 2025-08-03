mod api_character;
mod api_post;

use std::io::{BufReader, Read};
use actix_multipart::form::tempfile::TempFile;
use actix_web::web;
use serde::Serialize;
use std::path::PathBuf;
use tokio::fs;
use tracing::{error, info, warn};

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

async fn save_file(target_dir: &PathBuf, temp_file: Option<TempFile>, file_name: &str) -> Result<(String, String), ()> {
    let md5sum;

    if let Some(f) = temp_file {
        if !target_dir.exists() {
            if let Err(e) = fs::create_dir_all(&target_dir).await {
                error!("Failed to create file folder {:?}: {}", &target_dir.to_str(), e);
                return Err(());
            }
        }

        let mut hasher = Md5::new();
        let temp_file_name = f.file_name.unwrap_or_default();
        let extension = Path::new(temp_file_name.as_str())
            .extension()
            .unwrap_or(OsStr::new("png"))
            .to_str()
            .unwrap_or("png");
        let open_f = f.file.reopen().map_err(|_| {
            error!("Failed to reopen temp file");
            ()
        })?;
        let mut reader = BufReader::new(open_f);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).map_err(|_| {
            error!("Failed to read temp file");
            ()
        })?;
        hasher.update(&buf);
        md5sum = format!("{:x}", hasher.finalize());

        let saved_name = if file_name.is_empty() {
            format!("{}.{}", md5sum, extension)
        } else {
            String::from(file_name)
        };

        let path = target_dir.join(saved_name.as_str());
        if !path.exists() {
            if let Err(e) = f.file.persist(&path) {
                warn!("Failed to save file to {:?}: {}", &path.to_str(), e);
                warn!("Try copying");
                if let Err(e) = fs::copy(e.file.path(), &path).await {
                    error!("Failed to copy file: {}", e);
                    return Err(());
                }
            }
        } else {
            info!("File {saved_name} exists");
        }

        return Ok((md5sum, saved_name));
    }
    Ok((String::new(), String::new()))
}

async fn delete_file(root_dir: &PathBuf, file_path: String) {
    if !file_path.is_empty() {
        let real_path = root_dir.join(&file_path);
        if let Err(e) = fs::remove_file(&real_path).await {
            error!("Failed to remove file {}: {}", file_path, e);
        }
    }
}

async fn save_avatar(root_dir: &PathBuf, avatar: Option<TempFile>, file_name: &str) {
    let path = root_dir.join("avatar");
    if save_file(&path, avatar, file_name).await.is_ok() {
        info!("Saved avatar successfully");
    }
}
