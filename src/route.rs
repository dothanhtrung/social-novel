use actix_multipart::form::tempfile::TempFile;
use std::fs;
use std::path::PathBuf;
use md5::Md5;

pub mod character;
mod index;

macro_rules! redirect {
    ($url: expr) => {
        HttpResponse::Found().append_header(("Location", $url)).finish()
    };
}

pub(crate) use redirect;

pub fn save_file(target_dir: &PathBuf, temp_file: Option<TempFile>, file_name: &str) -> Result<String, ()> {
    let mut md5 = String::new();
    // TODO: Name file by checksum
    if let Some(f) = temp_file {
        if !target_dir.exists() {
            if let Err(e) = fs::create_dir_all(&target_dir) {
                log::error!("Failed to create file folder {:?}: {}", &target_dir.to_str(), e);
                return Err(());
            }
        }

        let path = target_dir.join(file_name);
        if let Err(e) = f.file.persist(&path) {
            log::warn!("Failed to save file to {:?}: {}", &path.to_str(), e);
            log::warn!("Try copying");
            if let Err(e) = fs::copy(e.file.path(), &path) {
                log::error!("Failed to copy file: {}", e);
                return Err(());
            }
        }
    }
    Ok(md5)
}
