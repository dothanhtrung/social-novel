use actix_multipart::form::tempfile::TempFile;
use md5::{Digest, Md5};
use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub mod character;
pub mod index;

macro_rules! redirect {
    ($url: expr) => {
        HttpResponse::Found().append_header(("Location", $url)).finish()
    };
}

pub(crate) use redirect;

pub fn save_file(target_dir: &PathBuf, temp_file: Option<TempFile>, file_name: &str) -> Result<String, ()> {
    let mut saved_name = String::from(file_name);

    if let Some(f) = temp_file {
        if !target_dir.exists() {
            if let Err(e) = fs::create_dir_all(&target_dir) {
                log::error!("Failed to create file folder {:?}: {}", &target_dir.to_str(), e);
                return Err(());
            }
        }

        if file_name.is_empty() {
            let mut hasher = Md5::new();
            let open_f = f.file.reopen().map_err(|_| {
                log::error!("Failed to reopen temp file");
                ()
            })?;
            let mut reader = BufReader::new(open_f);
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf).map_err(|_| {
                log::error!("Failed to read temp file");
                ()
            })?;
            hasher.update(&buf);
            saved_name = format!("{:x}", hasher.finalize());
        }

        let path = target_dir.join(saved_name.as_str());
        if let Err(e) = f.file.persist(&path) {
            log::warn!("Failed to save file to {:?}: {}", &path.to_str(), e);
            log::warn!("Try copying");
            if let Err(e) = fs::copy(e.file.path(), &path) {
                log::error!("Failed to copy file: {}", e);
                return Err(());
            }
        }

        return Ok(saved_name);
    }
    Ok(String::new())
}
