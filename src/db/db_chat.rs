use serde::{Deserialize, Serialize};
use crate::db::{postgres, DBPool};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRoom {
    pub id: i64,
    pub name: String,
}

pub async fn get_by_id(db_pool: &DBPool, id: i64) -> Result<Vec<ChatRoom>, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_chat::get_by_id(&db_pool.pg_pool, id).await
}

pub async fn get_by_member(db_pool: &DBPool, member: i64) -> Result<Vec<ChatRoom>, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_chat::get_by_member(&db_pool.pg_pool, member).await
}