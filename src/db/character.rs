use serde::Serialize;
use sqlx::{Error, PgPool};

#[derive(Serialize)]
pub struct Character {
    pub username: String,
    pub name: String,
}

pub async fn get_all_characters(pool: &PgPool) -> Result<Vec<Character>, Error> {
    sqlx::query_as!(Character, r#"SELECT * FROM character ORDER BY username"#).fetch_all(pool).await
}

pub async fn add_character(pool: &PgPool, username: &str, name: &str) -> Result<(), Error> {
    sqlx::query!(r#"INSERT INTO character ( username, name ) VALUES ( $1, $2 ) ON CONFLICT DO NOTHING"#, username, name).execute(pool).await.map(|_| ())
}

pub async fn update_character(pool: &PgPool, username: &str, name: &str) -> Result<(), Error> {
    sqlx::query!(r#"UPDATE character SET name = $1 WHERE username = $2"#, name, username).execute(pool).await.map(|_| ())
}