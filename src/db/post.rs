use serde::Serialize;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::{Error, PgPool};

#[derive(sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub parent: Option<i32>,
    pub content: String,
    pub character: String,
    pub reaction: i32,
    pub media: String,
    pub created_at: PrimitiveDateTime,
}

pub async fn count_all(pool: &PgPool) -> Result<i64, Error> {
    let rec = sqlx::query!(r#"SELECT count(id) FROM post"#).fetch_one(pool).await?;
    Ok(rec.count.unwrap_or_default())
}

pub async fn get_by_page(pool: &PgPool, ipp: usize, page: usize) -> Result<Vec<Post>, Error> {
    let offset = ipp * (page - 1);
    sqlx::query_as!(
        Post,
        r#"SELECT * FROM post ORDER BY created_at DESC OFFSET $1 LIMIT $2"#,
        offset as i64,
        ipp as i64
    )
    .fetch_all(pool)
    .await
}

pub async fn add(
    pool: &PgPool,
    parent: Option<i32>,
    content: String,
    character: String,
    media: String,
) -> Result<(), Error> {
    if let Some(parent) = parent {
        sqlx::query!(
            r#"INSERT INTO post ( parent, content, character, media ) VALUES ( $1, $2, $3, $4 )"#,
            parent,
            content,
            character,
            media
        )
        .execute(pool)
        .await
        .map(|_| ())
    } else {
        sqlx::query!(
            r#"INSERT INTO post ( content, character, media ) VALUES ( $1, $2, $3 )"#,
            content,
            character,
            media
        )
        .execute(pool)
        .await
        .map(|_| ())
    }
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query!(r#"DELETE FROM post WHERE id = $1"#, id)
        .execute(pool)
        .await
        .map(|_| ())
}
