use crate::db::db_media::Media;
use sqlx::SqlitePool;

pub(crate) async fn insert(pool: &SqlitePool, media: &Media) -> Result<i64, sqlx::Error> {
    let file_type = media.file_type as i64;
    let id = sqlx::query!(
        "INSERT INTO media (url, file_type, blake3, post) VALUES (?, ?, ?, ?) RETURNING id",
        media.url,
        file_type,
        media.blake3,
        media.post
    )
    .fetch_one(pool)
    .await?
    .id;
    Ok(id)
}

pub(crate) async fn get_by_post(pool: &SqlitePool, post: i64) -> Result<Vec<Media>, sqlx::Error> {
    sqlx::query_as!(Media, "SELECT * FROM media WHERE post = ?", post)
        .fetch_all(pool)
        .await
}

pub(crate) async fn delete_by_post(pool: &SqlitePool, post: i64) -> Result<Vec<String>, sqlx::Error> {
    let urls = sqlx::query_scalar!("DELETE FROM media WHERE post = ? RETURNING url", post)
        .fetch_all(pool)
        .await?;
    Ok(urls)
}
