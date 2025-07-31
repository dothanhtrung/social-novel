#[cfg(feature = "sqlite")]
use crate::db::sqlite;
use crate::db::DBPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub content: String,
    pub author: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub parent: Option<i64>,
    pub liked: i64,
    pub haha: i64,
    pub loved: i64,
    pub surprised: i64,
}

pub async fn add(db_pool: &DBPool, post: Post) -> Result<i64, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    sqlite::post::add(&db_pool.sqlite_pool, post).await
}

pub async fn get(db_pool: &DBPool, id: i64) -> Result<Post, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    sqlite::post::get(&db_pool.sqlite_pool, id).await
}

pub async fn get_all(db_pool: &DBPool, limit: i64, offset: i64) -> Result<Vec<Post>, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    sqlite::post::get_all(&db_pool.sqlite_pool, limit, offset).await
}

pub async fn get_by_author(
    db_pool: &DBPool,
    author_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    sqlite::post::get_by_author(&db_pool.sqlite_pool, author_id, limit, offset).await
}

pub async fn get_by_parent(db_pool: &DBPool, parent_id: i64) -> Result<Vec<Post>, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    sqlite::post::get_by_parent(&db_pool.sqlite_pool, parent_id).await
}

pub async fn update(db_pool: &DBPool, post: Post) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    sqlite::post::update(&db_pool.sqlite_pool, post).await
}

pub async fn delete(db_pool: &DBPool, id: i64) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    sqlite::post::delete(&db_pool.sqlite_pool, id).await
}
