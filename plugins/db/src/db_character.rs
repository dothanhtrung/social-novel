#[cfg(feature = "postgres")]
use crate::postgres;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use web_misc::db::DBPool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub username: String,
    pub name: String,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub bio: Json<Bio>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Bio {
    pub love: Option<String>,
}

pub async fn search(db_pool: &DBPool, search: &str) -> Result<Vec<Character>, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_character::search(&db_pool.pg_pool, search).await
}

pub async fn insert(db_pool: &DBPool, character: &Character) -> Result<i64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_character::insert(&db_pool.pg_pool, character.name.as_str(), character.username.as_str()).await
}

pub async fn get(db_pool: &DBPool, id: i64) -> Result<Character, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_character::get(&db_pool.pg_pool, id).await
}

pub async fn get_by_username(db_pool: &DBPool, username: &str) -> Result<Character, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_character::get_by_username(&db_pool.pg_pool, username).await
}

pub async fn delete(db_pool: &DBPool, id: i64) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_character::delete(&db_pool.pg_pool, id).await
}

pub async fn update(dbpool: &DBPool, character: &Character) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_character::update(
        &dbpool.pg_pool,
        character.id,
        character.name.as_str(),
        character.username.as_str(),
        character.description.as_str(),
    )
    .await
}
