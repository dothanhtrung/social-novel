use crate::db::post;
use sqlx::{Error, PgPool};

// #[derive(sqlx::FromRow)]
// pub struct Media {
//     pub md5sum: String,
//     pub path: String,
//     pub created_at: PrimitiveDateTime,
// }

pub async fn add(pool: &PgPool, md5sum: &str, path: &str) -> Result<(), Error> {
    sqlx::query!(r#"INSERT INTO media ( md5sum, path ) VALUES ( $1, $2 ) ON CONFLICT DO NOTHING"#, md5sum, path)
        .execute(pool)
        .await
        .map(|_| ())
}

pub async fn force_delete(pool: &PgPool, md5sum: &str) -> Result<(), Error> {
    post::remove_media(pool, md5sum).await?;
    sqlx::query!(r#"DELETE FROM media WHERE md5sum = $1"#, md5sum)
        .execute(pool)
        .await
        .map(|_| ())
}

pub async fn delete_if_no_use(pool: &PgPool, md5sum: &str) -> Result<String, Error> {
    if let Ok(count) = post::count_by_media(pool, md5sum).await {
        if count <= 0 {
            let rec = sqlx::query!(r#"SELECT path FROM media WHERE md5sum = $1"#, md5sum).fetch_one(pool).await?;
            force_delete(pool, md5sum).await?;
            return Ok(rec.path);
        }
    }
    Ok(String::new())
}

pub async fn delete(pool: &PgPool, md5sum: &str) -> Result<String, Error> {
    delete_if_no_use(pool, md5sum).await
}
