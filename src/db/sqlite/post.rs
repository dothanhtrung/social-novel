use crate::db::post::Post;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::SqlitePool;

pub(crate) async fn add(pool: &SqlitePool, post: Post) -> Result<i64, sqlx::Error> {
    let id = sqlx::query!(
        "INSERT INTO post (content, author, parent, liked, haha, loved, surprised) VALUES (?,?,?,?,?,?,?) RETURNING id",
        post.content,
        post.author,
        post.parent,
        post.liked,
        post.haha,
        post.loved,
        post.surprised
    )
    .fetch_one(pool)
    .await?
    .id;
    Ok(id)
}

pub(crate) async fn get(pool: &SqlitePool, id: i64) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(Post, r#"SELECT * FROM post  WHERE id = ?"#, id)
        .fetch_one(pool)
        .await
}

pub(crate) async fn get_all(pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"SELECT * FROM post  ORDER BY updated_at DESC LIMIT ? OFFSET ?"#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn get_by_author(
    pool: &SqlitePool,
    author_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"SELECT * FROM post  WHERE author = ? ORDER BY updated_at DESC LIMIT ? OFFSET ?"#,
        author_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn get_by_parent(pool: &SqlitePool, parent_id: i64) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"SELECT * FROM post  WHERE parent = ? ORDER BY updated_at"#,
        parent_id
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn update(pool: &SqlitePool, post: Post) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        "UPDATE post SET content = ?, author = ?, parent = ?, liked = ?, haha = ?, loved = ?, surprised = ?, updated_at = ? WHERE id = ?",
        post.content,
        post.author,
        post.parent,
        post.liked,
        post.haha,
        post.loved,
        post.surprised,
        post.updated_at,
        post.id
    ).execute(pool).await
}

pub(crate) async fn delete(pool: &SqlitePool, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM post WHERE id = ?", id).execute(pool).await
}
