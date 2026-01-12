use crate::db::db_media::Media;
use sqlx::PgPool;

pub(crate) async fn insert(pool: &PgPool, media: &Media) -> Result<i64, sqlx::Error> {
    let file_type = media.file_type as i16;
    let id = sqlx::query!(
        "INSERT INTO media (url, file_type, blake3, post) VALUES ($1, $2, $3, $4) RETURNING id",
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

pub(crate) async fn get_by_post(pool: &PgPool, post: i64) -> Result<Vec<Media>, sqlx::Error> {
    sqlx::query_as!(Media, "SELECT * FROM media WHERE post = $1", post)
        .fetch_all(pool)
        .await
}

pub(crate) async fn delete_by_post(pool: &PgPool, post: i64) -> Result<Vec<String>, sqlx::Error> {
    let urls = sqlx::query_scalar!("DELETE FROM media WHERE post = $1 RETURNING url", post)
        .fetch_all(pool)
        .await?;
    Ok(urls)
}
