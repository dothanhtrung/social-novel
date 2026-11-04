#[cfg(feature = "postgres")]
use crate::postgres;
use serde::Serialize;
use web_misc::db::DBPool;

/// The other character in an undirected friendship, plus the latest message
/// from the chat room that contains only those two characters.
#[derive(Serialize, Debug, Clone)]
pub struct Friend {
    pub id: i64,
    pub username: String,
    pub name: String,
    /// `0` when those two characters do not yet share a two-person room.
    pub room_id: i64,
    pub last_message: String,
    pub last_author: i64,
    /// Empty when there is no message. `YYYY-MM-DD HH:MM:SS` in UTC otherwise.
    pub last_at: String,
}

pub async fn list(db_pool: &DBPool, character_id: i64) -> Result<Vec<Friend>, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_friend::list(&db_pool.pg_pool, character_id).await
}

pub async fn insert(db_pool: &DBPool, character_id: i64, friend_id: i64) -> Result<i64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_friend::insert(&db_pool.pg_pool, character_id, friend_id).await
}

pub async fn delete_pair(db_pool: &DBPool, character_id: i64, friend_id: i64) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_friend::delete_pair(&db_pool.pg_pool, character_id, friend_id).await
}
