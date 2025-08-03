use crate::db::db_media::Media;
use sqlx::SqlitePool;

pub(crate) async fn insert(pool: &SqlitePool, media: &Media) -> Result<i64, sqlx::Error> {
    let file_type = media.file_type as i64;
    let id = sqlx::query!(
        "INSERT INTO media (url, file_type, post) VALUES (?, ?, ?) RETURNING id",
        media.url,
        file_type,
        media.post
    )
    .fetch_one(pool)
    .await?
    .id;
    Ok(id)
}
