use web_misc::db::DBPool;
#[cfg(feature = "postgres")]
use crate::postgres;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub description: String,
}

pub async fn insert(dbpool: &DBPool, group: &Group) -> Result<i64, anyhow::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_group::insert(&dbpool.pg_pool, group)
        .await
        .map_err(|e| e.into());
}

pub async fn search(dbpool: &DBPool, search: &str, member: Option<i64>) -> Result<Vec<Group>, anyhow::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_group::search(&dbpool.pg_pool, search, member)
        .await
        .map_err(|e| e.into())
}

/// A character who has joined a group.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMember {
    pub id: i64,
    pub username: String,
    pub name: String,
}

pub async fn members(dbpool: &DBPool, group_id: i64) -> Result<Vec<GroupMember>, anyhow::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_group::members(&dbpool.pg_pool, group_id)
        .await
        .map_err(|e| e.into())
}

pub async fn join(dbpool: &DBPool, character_id: i64, group_id: i64) -> Result<u64, anyhow::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_group::join(&dbpool.pg_pool, character_id, group_id)
        .await
        .map_err(|e| e.into())
}

pub async fn leave(dbpool: &DBPool, character_id: i64, group_id: i64) -> Result<u64, anyhow::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_group::leave(&dbpool.pg_pool, character_id, group_id)
        .await
        .map_err(|e| e.into())
}

pub async fn update(dbpool: &DBPool, group: &Group) -> Result<(), anyhow::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_group::update(&dbpool.pg_pool, group)
        .await
        .map_err(|e| e.into());
}

pub async fn delete_by_id(dbpool: &DBPool, group_id: i64) -> Result<u64, anyhow::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_group::delete_by_id(&dbpool.pg_pool, group_id)
        .await
        .map_err(|e| e.into());
}
