use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Character {
    pub username: String,
    pub name: String,
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<Character>, sqlx::Error> {
    sqlx::query_as!(Character, r#"SELECT * FROM character ORDER BY username"#).fetch_all(pool).await
}