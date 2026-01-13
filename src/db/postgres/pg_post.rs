use crate::db::db_post::Post;
use sqlx::PgPool;

pub(crate) async fn insert(pool: &PgPool, post: &Post) -> Result<i64, sqlx::Error> {
    let id = sqlx::query!(
        "INSERT INTO post (content, author, parent, liked, haha, loved, surprised, sad, feeling, is_with) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) RETURNING id",
        post.content,
        post.author,
        post.parent,
        post.liked,
        post.haha,
        post.loved,
        post.surprised,
        post.sad,
        post.feeling,
        post.is_with,
    )
        .fetch_one(pool)
        .await?
        .id;
    Ok(id)
}

pub(crate) async fn get_by_id(pool: &PgPool, id: i64) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(Post, r#"SELECT * FROM post WHERE id = $1"#, id)
        .fetch_one(pool)
        .await
}

pub(crate) async fn get_all(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"SELECT * FROM post ORDER BY updated_at DESC LIMIT $1 OFFSET $2"#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn get_by_author(
    pool: &PgPool,
    author_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"SELECT * FROM post WHERE author = $1 ORDER BY updated_at DESC LIMIT $2 OFFSET $3"#,
        author_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn get_by_parent(pool: &PgPool, parent_id: i64) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"SELECT * FROM post WHERE parent = $1 ORDER BY updated_at"#,
        parent_id
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn update(pool: &PgPool, post: &Post) -> Result<u64, sqlx::Error> {
    // TODO: Update updated_at
    let count = sqlx::query!(
        "UPDATE post SET content = $1, author = $2, parent = $3, liked = $4, haha = $5, loved = $6, surprised = $7, sad = $8, feeling = $9, is_with = $10 WHERE id = $11",
        post.content,
        post.author,
        post.parent,
        post.liked,
        post.haha,
        post.loved,
        post.surprised,
        post.sad,
        post.feeling,
        post.is_with,
        post.id
    ).execute(pool).await?.rows_affected();

    Ok(count)
}

pub(crate) async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!("DELETE FROM post WHERE id = $1", id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}
