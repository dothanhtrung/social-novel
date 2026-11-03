#[cfg(feature = "postgres")]
use crate::postgres;
use web_misc::db::DBPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(i16)]
pub enum MediaType {
    NA = 0,
    Image = 1,
    Video = 2,
    Audio = 3,
}

impl From<i16> for MediaType {
    fn from(value: i16) -> Self {
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
    pub blake3: String,
}

/// Store the file if needed and link it to the post. An existing url is reused.
pub async fn attach(dbpool: &DBPool, post_id: i64, media: &Media) -> Result<i64, anyhow::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_media::attach(&dbpool.pg_pool, post_id, media)
        .await
        .map_err(|e| e.into());
}

pub async fn get_by_post(dbpool: &DBPool, post_id: i64) -> Result<Vec<Media>, anyhow::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_media::get_by_post(&dbpool.pg_pool, post_id)
        .await
        .map_err(|e| e.into())
}

/// Remove links for these posts. Returns urls of media no longer used by any post.
pub async fn unlink_posts(dbpool: &DBPool, post_ids: &[i64]) -> Result<Vec<String>, anyhow::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_media::unlink_posts(&dbpool.pg_pool, post_ids)
        .await
        .map_err(|e| e.into());
}

pub async fn unlink_group(dbpool: &DBPool, group_id: i64) -> Result<Vec<String>, anyhow::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_media::unlink_group(&dbpool.pg_pool, group_id)
        .await
        .map_err(|e| e.into());
}

pub async fn unlink_room(dbpool: &DBPool, room_id: i64) -> Result<Vec<String>, anyhow::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_media::unlink_room(&dbpool.pg_pool, room_id)
        .await
        .map_err(|e| e.into());
}
