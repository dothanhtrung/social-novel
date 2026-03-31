//! Copyright (c) 2025 Trung Do <dothanhtrung@pm.me>.

pub mod db_ads;
pub mod db_character;
pub mod db_chat;
pub mod db_group;
pub mod db_media;
pub mod db_post;
#[cfg(feature = "postgres")]
mod postgres;

use crate::config::DBConfig;
#[cfg(feature = "postgres")]
use sqlx::{postgres::PgConnectOptions, PgPool};
use std::str::FromStr;

pub struct DBPool {
    #[cfg(feature = "postgres")]
    pub pg_pool: PgPool,
}

impl DBPool {
    pub async fn init(config: &DBConfig) -> anyhow::Result<Self> {
        #[cfg(feature = "postgres")]
        let pg_pool = {
            let pg_opts = PgConnectOptions::from_str(&config.postgres.db_path)?;
            let pool = PgPool::connect_with(pg_opts).await?;
            sqlx::migrate!("./migrations/postgres").run(&pool).await?;
            pool
        };

        Ok(Self {
            #[cfg(feature = "postgres")]
            pg_pool,
        })
    }
}
