//! Copyright (c) 2025 Trung Do <dothanhtrung@pm.me>.

#[cfg(feature = "sqlite")]
mod sqlite;
#[cfg(feature = "postgres")]
mod postgres;
pub mod db_character;
pub mod db_post;
pub mod db_media;

#[cfg(feature = "sqlite")]
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
#[cfg(feature = "sqlite")]
use sqlx::SqlitePool;
use std::str::FromStr;
#[cfg(feature = "postgres")]
use sqlx::{PgPool,postgres::PgConnectOptions};
use crate::config::DBConfig;

pub struct DBPool {
    #[cfg(feature = "sqlite")]
    pub sqlite_pool: SqlitePool,
    #[cfg(feature = "postgres")]
    pub pg_pool: PgPool,
}

impl DBPool {
    pub async fn init(config: &DBConfig) -> anyhow::Result<Self> {
        #[cfg(feature = "sqlite")]
        let sqlite_pool = {
            let opts = SqliteConnectOptions::from_str(&config.sqlite.db_path)?
                .foreign_keys(true)
                .journal_mode(SqliteJournalMode::Wal)
                .create_if_missing(true);
            let pool = SqlitePool::connect_with(opts).await?;
            sqlx::migrate!("./migrations/sqlite").run(&pool).await?;
            pool
        };

        #[cfg(feature = "postgres")]
        let pg_pool = {
            let pg_opts = PgConnectOptions::from_str(&config.postgres.db_path)?;
            let pool = PgPool::connect_with(pg_opts).await?;
            sqlx::migrate!("./migrations/postgres").run(&pool).await?;
            pool
        };

        Ok(Self {
            #[cfg(feature = "sqlite")]
            sqlite_pool,
            #[cfg(feature = "postgres")]
            pg_pool,
        })
    }
}
