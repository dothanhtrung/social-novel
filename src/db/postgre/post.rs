
use sqlx::postgres::PgQueryResult;
use sqlx::types::time::{PrimitiveDateTime};
use sqlx::{Error, PgPool};

pub async fn count_all(pool: &PgPool) -> Result<i64, Error> {
    let rec = sqlx::query!(r#"SELECT count(id) FROM post"#).fetch_one(pool).await?;
    Ok(rec.count.unwrap_or_default())
}

pub async fn get_by_page(pool: &PgPool, ipp: i64, page: i64) -> Result<Vec<Post>, Error> {
    let offset = ipp * (page - 1);
    sqlx::query_as!(
        Post,
        r#"SELECT id, content, character, character.name as character_name, reaction, media.path as media, post.created_at as created_at, feeling, is_with FROM post LEFT JOIN character ON post.character = character.username LEFT JOIN media ON post.media = media.md5sum WHERE parent is NULL ORDER BY post.created_at DESC OFFSET $1 LIMIT $2"#,
        offset as i64,
        ipp as i64
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_parent(pool: &PgPool, parent: i32) -> Result<Vec<Post>, Error> {
    sqlx::query_as!(
        Post,
        r#"SELECT id, content, character, character.name as character_name, reaction, media.path as media, post.created_at as created_at, feeling, is_with FROM post LEFT JOIN character ON post.character = character.username LEFT JOIN media ON post.media = media.md5sum WHERE parent = $1 ORDER BY created_at ASC"#,
        parent
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
    reaction: i32,
    is_with: String,
    feeling: String,
) -> Result<(), Error> {
    if let Some(parent) = parent {
        sqlx::query!(
            r#"INSERT INTO post ( parent, content, character, media, reaction, is_with, feeling ) VALUES ( $1, $2, $3, $4, $5, $6, $7 )"#,
            parent,
            content,
            character,
            media,
            reaction,
            is_with,
            feeling,
        )
        .execute(pool)
        .await
        .map(|_| ())
    } else {
        sqlx::query!(
            r#"INSERT INTO post ( content, character, media, reaction, is_with, feeling ) VALUES ( $1, $2, $3, $4, $5, $6 )"#,
            content,
            character,
            media,
            reaction,
            is_with,
            feeling,
        )
        .execute(pool)
        .await
        .map(|_| ())
    }
}

pub async fn update<S: AsRef<str>>(
    pool: &PgPool,
    character: S,
    content: S,
    media: S,
    reaction: i32,
) -> Result<(), Error> {
    sqlx::query!(
        r#"UPDATE post SET character = $1, content = $2, media = $3, reaction = $4"#,
        character.as_ref(),
        content.as_ref(),
        media.as_ref(),
        reaction
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query!(r#"DELETE FROM post WHERE parent = $1"#, id)
        .execute(pool)
        .await?;
    sqlx::query!(r#"DELETE FROM post WHERE id = $1"#, id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn remove_media<S: AsRef<str>>(pool: &PgPool, md5sum: S) -> Result<PgQueryResult, Error> {
    sqlx::query!(r#"UPDATE post SET media = NULL WHERE media = $1"#, md5sum.as_ref())
        .execute(pool)
        .await
}

pub(crate) async fn count_by_media(pool: &PgPool, md5sum: &str) -> Result<i64, Error> {
    let rec = sqlx::query!(r#"SELECT count(id) FROM post WHERE media = $1"#, md5sum).fetch_one(pool).await?;
    Ok(rec.count.unwrap_or_default())
}

pub async fn get_media(pool: &PgPool, id: i32) -> Result<String, Error> {
    let rec = sqlx::query!(r#"SELECT media FROM post WHERE id = $1"#, id).fetch_one(pool).await?;
    Ok(rec.media.unwrap_or_default())
}