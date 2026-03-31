use crate::db::db_character::Character;
use sqlx::types::Json;
use sqlx::PgPool;
use super::super::db_character::Bio;

pub(crate) async fn search(pool: &PgPool, search: &str) -> Result<Vec<Character>, sqlx::Error> {
    sqlx::query_as!(
        Character,
        r#"SELECT id, name, username, description, bio AS "bio: Json<Bio>"
            FROM character
            WHERE name ILIKE '%' || $1 || '%' OR username ILIKE '%' || $2 || '%'
            ORDER BY name"#,
        search,
        search
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn insert(pool: &PgPool, name: &str, username: &str) -> Result<i64, sqlx::Error> {
    let id = sqlx::query_scalar!(
        "INSERT INTO character (name, username) VALUES ($1, $2) RETURNING id",
        name,
        username
    )
    .fetch_one(pool)
    .await?;
    Ok(id)
}

pub(crate) async fn update(
    pool: &PgPool,
    id: i64,
    name: &str,
    username: &str,
    description: &str,
) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(
        "UPDATE character SET name = $1, username = $2, description = $3 WHERE id = $4",
        name,
        username,
        description,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(count)
}

pub(crate) async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let affected_rows = sqlx::query!("DELETE FROM character WHERE id = $1", id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(affected_rows)
}

pub(crate) async fn get(pool: &PgPool, id: i64) -> Result<Character, sqlx::Error> {
    sqlx::query_as!(Character, r#"SELECT id, name, username, description, bio AS "bio: Json<Bio>"
                                    FROM character WHERE id = $1"#, id)
        .fetch_one(pool)
        .await
}

pub(crate) async fn get_by_username(pool: &PgPool, username: &str) -> Result<Character, sqlx::Error> {
    sqlx::query_as!(Character, r#"SELECT id, name, username, description, bio AS "bio: Json<Bio>"
                                    FROM character WHERE username = $1"#, username)
        .fetch_one(pool)
        .await
}
