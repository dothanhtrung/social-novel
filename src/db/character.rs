use serde::Serialize;
use sqlx::{Error, PgPool};

#[derive(Serialize)]
pub struct Character {
    pub username: String,
    pub name: String,
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<Character>, Error> {
    sqlx::query_as!(Character, r#"SELECT * FROM character ORDER BY username ASC"#)
        .fetch_all(pool)
        .await
}

pub async fn add(pool: &PgPool, username: &str, name: &str) -> Result<(), Error> {
    sqlx::query!(
        r#"INSERT INTO character ( username, name ) VALUES ( $1, $2 ) ON CONFLICT DO NOTHING"#,
        username,
        name
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn update(pool: &PgPool, username: &str, name: &str) -> Result<(), Error> {
    sqlx::query!(r#"UPDATE character SET name = $1 WHERE username = $2"#, name, username)
        .execute(pool)
        .await
        .map(|_| ())
}

pub async fn get(pool: &PgPool, username: &str) -> Result<Character, Error> {
    sqlx::query_as!(Character, r#"SELECT * FROM character WHERE username = $1"#, username)
        .fetch_one(pool)
        .await
}

pub async fn delete(pool: &PgPool, username: &str) -> Result<(), Error> {
    sqlx::query!(r#"DELETE FROM character WHERE username = $1"#, username)
        .execute(pool)
        .await
        .map(|_| ())
}
