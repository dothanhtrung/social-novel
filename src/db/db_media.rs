#[cfg(feature = "sqlite")]
use crate::db::sqlite;
use crate::db::DBPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(i64)]
pub enum MediaType {
    NA = 0,
    Image = 1,
    Video = 2,
    Audio = 3,
}

impl From<i64> for MediaType {
    fn from(value: i64) -> Self {
        match value {
            0 => MediaType::NA,
            1 => MediaType::Image,
            2 => MediaType::Video,
            3 => MediaType::Audio,
            _ => MediaType::NA,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Media {
    #[serde(default)]
    pub id: i64,
    pub url: String,
    pub file_type: MediaType,
    pub post: i64,
    pub blake3: String,
}

pub async fn insert(dbpool: &DBPool, media: &Media) -> Result<i64, anyhow::Error> {
    #[cfg(feature = "sqlite")]
    sqlite::sqlite_media::insert(&dbpool.sqlite_pool, media)
        .await
        .map_err(|e| e.into())
}

pub async fn get_by_post(dbpool: &DBPool, post_id: i64) -> Result<Vec<Media>, anyhow::Error> {
    #[cfg(feature = "sqlite")]
    sqlite::sqlite_media::get_by_post(&dbpool.sqlite_pool, post_id)
        .await
        .map_err(|e| e.into())
}

pub async fn delete_by_post(dbpool: &DBPool, post_id: i64) -> Result<Vec<String>, anyhow::Error> {
    #[cfg(feature = "sqlite")]
    sqlite::sqlite_media::delete_by_post(&dbpool.sqlite_pool, post_id)
        .await
        .map_err(|e| e.into())
}
