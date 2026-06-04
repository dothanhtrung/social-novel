#[cfg(feature = "postgres")]
use crate::postgres;
use web_misc::db::DBPool;
use actix_web::cookie::time::format_description::well_known::Iso8601;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::types::time::OffsetDateTime;
use tracing::{info, warn};

#[derive(Serialize, Deserialize)]
pub struct Post {
    #[serde(default)]
    pub id: i64,
    pub content: String,
    pub author: i64,
    #[serde(
        serialize_with = "offsetdatetime_serialize",
        deserialize_with = "offsetdatetime_deserialize"
    )]
    pub created_at: OffsetDateTime,
    #[serde(
        serialize_with = "offsetdatetime_serialize",
        deserialize_with = "offsetdatetime_deserialize"
    )]
    pub updated_at: OffsetDateTime,
    pub parent: Option<i64>,
    #[serde(default)]
    pub liked_by: String,
    #[serde(default)]
    pub liked: i32,
    #[serde(default)]
    pub haha: i32,
    #[serde(default)]
    pub loved: i32,
    #[serde(default)]
    pub surprised: i32,
    #[serde(default)]
    pub sad: i32,
    #[serde(default)]
    pub feeling: String,
    #[serde(default)]
    pub is_with: String,
    #[serde(default)]
    pub group: Option<i64>,
    #[serde(default)]
    pub room: Option<i64>,
}

#[derive(Default)]
pub struct SearchPostCondition {
    pub authors: Option<Vec<i64>>,
    pub groups: Option<Vec<i64>>,
    pub rooms: Option<Vec<i64>>,
}

pub async fn insert(db_pool: &DBPool, post: &Post) -> Result<i64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_post::insert(&db_pool.pg_pool, post).await;
}

pub async fn get_by_id(db_pool: &DBPool, id: i64) -> Result<Post, sqlx::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_post::get_by_id(&db_pool.pg_pool, id).await;
}

pub async fn get_all(db_pool: &DBPool, limit: i64, offset: i64, cond: &SearchPostCondition) -> Result<Vec<Post>, sqlx::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_post::get_all(&db_pool.pg_pool, limit, offset, cond).await;
}

pub async fn get_by_parent(db_pool: &DBPool, parent_id: i64) -> Result<Vec<Post>, sqlx::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_post::get_by_parent(&db_pool.pg_pool, parent_id).await;
}

pub async fn update(db_pool: &DBPool, post: &Post) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_post::update(&db_pool.pg_pool, post).await;
}

pub async fn delete_by_id(db_pool: &DBPool, id: i64) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_post::delete_by_id(&db_pool.pg_pool, id).await;
}

pub async fn delete_by_group(db_pool: &DBPool, group_id: i64) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_post::delete_by_group(&db_pool.pg_pool, group_id).await;
}

pub async fn delete_by_room(db_pool: &DBPool, room_id: i64) -> Result<u64, sqlx::Error> {
    #[cfg(feature = "postgres")]
    return postgres::pg_post::delete_by_room(&db_pool.pg_pool, room_id).await;
}

fn offsetdatetime_serialize<S>(offset_datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(offset_datetime.to_string().as_str())
}
fn offsetdatetime_deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match string_to_offsetdatetime(s) {
        Ok(dt) => Ok(dt),
        Err(e) => Err(Error::custom(format!("Failed to parse datetime string: {}", e))),
    }
}

pub fn string_to_offsetdatetime(time_str: &str) -> Result<OffsetDateTime, anyhow::Error> {
    match OffsetDateTime::parse(time_str, &Iso8601::DEFAULT) {
        Ok(ret) => return Ok(ret),
        Err(e) => info!(
            "Failed to parse \"{}\" in Iso8601 format: {}. Try another format.",
            time_str, e
        ),
    }

    warn!("No suitable date time format found for \"{time_str}\"");
    Err(anyhow::anyhow!("Invalid date time format: {time_str}"))
}
