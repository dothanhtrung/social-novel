#[cfg(feature = "postgres")]
use crate::postgres;
use serde::{Deserialize, Serialize};
use web_misc::db::DBPool;

/// A directed relationship: `from_character` is `relation` of `to_character`.
/// Example: Himawari (from) is daughter (relation) of Hinata (to).
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Relation {
    #[serde(default)]
    pub id: i64,
    pub from_character: i64,
    pub to_character: i64,
    pub relation: String,
    #[serde(default)]
    pub from_name: String,
    #[serde(default)]
    pub from_username: String,
    #[serde(default)]
    pub to_name: String,
    #[serde(default)]
    pub to_username: String,
}

pub struct RelationCond {
    /// Either side of the relationship. `None` lists every relation.
    pub character: Option<i64>,
}

pub async fn search(db_pool: &DBPool, cond: &RelationCond) -> Result<Vec<Relation>, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_relation::search(&db_pool.pg_pool, cond).await
}

pub async fn get(db_pool: &DBPool, id: i64) -> Result<Relation, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_relation::get(&db_pool.pg_pool, id).await
}

pub async fn insert(db_pool: &DBPool, relation: &Relation) -> Result<i64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_relation::insert(&db_pool.pg_pool, relation).await
}

pub async fn update(db_pool: &DBPool, relation: &Relation) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_relation::update(&db_pool.pg_pool, relation).await
}

pub async fn delete(db_pool: &DBPool, id: i64) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_relation::delete(&db_pool.pg_pool, id).await
}
