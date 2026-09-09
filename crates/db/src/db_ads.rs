use serde::{Deserialize, Serialize};
use web_misc::db::DBPool;

#[cfg(feature = "postgres")]
use crate::postgres;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Ads {
    pub id: i64,
    pub description: String,
    pub active: bool,
    #[serde(default)]
    pub media: String,
}

pub struct AdsCond {
    pub active: Option<bool>,
}

pub async fn search(db_pool: &DBPool, cond: &AdsCond) -> Result<Vec<Ads>, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_ads::search(&db_pool.pg_pool, cond).await
}

pub async fn get(db_pool: &DBPool, id: i64) -> Result<Ads, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_ads::get(&db_pool.pg_pool, id).await
}

pub async fn insert(db_pool: &DBPool, ads: &Ads) -> Result<i64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_ads::insert(&db_pool.pg_pool, ads).await
}

pub async fn update(db_pool: &DBPool, ads: &Ads) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_ads::update(&db_pool.pg_pool, ads).await
}

pub async fn delete(db_pool: &DBPool, id: i64) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    postgres::pg_ads::delete(&db_pool.pg_pool, id).await
}
