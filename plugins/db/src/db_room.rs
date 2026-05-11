#[cfg(feature = "postgres")]
use crate::postgres;
use serde::{Deserialize, Serialize};
use web_misc::db::DBPool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRoom {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct RoomQuery {
    pub page: Option<i64>,
    pub count: Option<i64>,
    pub id: Option<Vec<i64>>,
    pub member: Option<Vec<i64>>,
}

#[derive(Deserialize)]
pub struct RoomMember {
    pub room: i64,
    pub member: i64,
}

pub async fn search(db_pool: &DBPool, query: &RoomQuery) -> Result<Vec<ChatRoom>, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_room::search(&db_pool.pg_pool, &query).await
}

pub async fn delete_by_id(db_pool: &DBPool, id: i64) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_room::delete_by_id(&db_pool.pg_pool, id).await
}

pub async fn insert(db_pool: &DBPool, room_info: &ChatRoom) -> Result<i64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_room::insert(&db_pool.pg_pool, room_info).await
}

pub async fn update(db_pool: &DBPool, room_info: &ChatRoom) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_room::update(&db_pool.pg_pool, room_info).await
}

pub async fn invite(db_pool: &DBPool, info: &RoomMember) -> Result<(), sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_room::invite(&db_pool.pg_pool, info).await
}

pub async fn kick(db_pool: &DBPool, info: &RoomMember) -> Result<(), sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_room::kick(&db_pool.pg_pool, info).await
}
