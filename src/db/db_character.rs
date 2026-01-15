#[cfg(feature = "sqlite")]
use crate::db::sqlite;
use crate::db::{postgres, DBPool};
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub struct Character {
    pub username: String,
    pub name: String,
    #[serde(default)]
    pub id: i64,
}

pub async fn search(db_pool: &DBPool, search: &str) -> Result<Vec<Character>, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    return sqlite::sqlite_character::search(&db_pool.sqlite_pool, search).await;

    #[cfg(feature = "postgres")]
    postgres::pg_character::search(&db_pool.pg_pool, search).await
}

pub async fn insert(db_pool: &DBPool, character: &Character) -> Result<i64, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    return sqlite::sqlite_character::insert(
        &db_pool.sqlite_pool,
        character.name.as_str(),
        character.username.as_str(),
    )
        .await;

    #[cfg(feature = "postgres")]
    postgres::pg_character::insert(
        &db_pool.pg_pool,
        character.name.as_str(),
        character.username.as_str(),
    )
        .await
}

pub async fn get(db_pool: &DBPool, id: i64) -> Result<Character, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    return sqlite::sqlite_character::get(&db_pool.sqlite_pool, id).await;

    #[cfg(feature = "postgres")]
    postgres::pg_character::get(&db_pool.pg_pool, id).await
}

pub async fn delete(db_pool: &DBPool, id: i64) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    return sqlite::sqlite_character::delete(&db_pool.sqlite_pool, id).await;

    #[cfg(feature = "postgres")]
    postgres::pg_character::delete(&db_pool.pg_pool, id).await
}

pub async fn update(dbpool: &DBPool, character: &Character) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "sqlite")]
    return sqlite::sqlite_character::update(
        &dbpool.sqlite_pool,
        character.id,
        character.name.as_str(),
        character.username.as_str(),
    )
        .await;

    #[cfg(feature = "postgres")]
    postgres::pg_character::update(
        &dbpool.pg_pool,
        character.id,
        character.name.as_str(),
        character.username.as_str(),
    )
        .await
}
