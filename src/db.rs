//! Copyright (c) 2025 Trung Do <dothanhtrung@pm.me>.

#[cfg(feature = "sqlite")]
mod sqlite;
pub mod db_character;
pub mod db_post;

#[cfg(feature = "sqlite")]
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
#[cfg(feature = "sqlite")]
use sqlx::SqlitePool;
use std::str::FromStr;
use crate::config::DBConfig;

pub struct DBPool {
    pub sqlite_pool: SqlitePool,
}

impl DBPool {
    pub async fn init(config: &DBConfig) -> anyhow::Result<Self> {
        #[cfg(feature = "sqlite")]
        {
            let opts = SqliteConnectOptions::from_str(&config.sqlite.db_path)?
                .foreign_keys(true)
                .journal_mode(SqliteJournalMode::Wal)
                .create_if_missing(true);
            let sqlite_pool = SqlitePool::connect_with(opts).await?;
            sqlx::migrate!("./migrations").run(&sqlite_pool).await?;

            Ok(Self { sqlite_pool })
        }
    }
}
