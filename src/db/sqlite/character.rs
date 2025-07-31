use crate::db::character::Character;
use sqlx::SqlitePool;

pub(crate) async fn search(pool: &SqlitePool, search: &str) -> Result<Vec<Character>, sqlx::Error> {
    sqlx::query_as!(
        Character,
        "SELECT * FROM character WHERE name LIKE '%' || ? || '%' OR username LIKE '%' || ? || '%' ORDER BY name",
        search,
        search
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn add(pool: &SqlitePool, name: &str, username: &str) -> Result<i64, sqlx::Error> {
    let id = sqlx::query!(
        "INSERT INTO character (name, username) VALUES (?, ?) RETURNING id",
        name,
        username
    )
    .fetch_one(pool)
    .await?
    .id;
    Ok(id)
}

pub(crate) async fn update(pool: &SqlitePool, id: i64, name: &str, username: &str) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(
        "UPDATE character SET name = ?, username = ? WHERE id = ?",
        name,
        username,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(count)
}

pub(crate) async fn delete(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    let affected_rows = sqlx::query!("DELETE FROM character WHERE id = ?", id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(affected_rows)
}

pub(crate) async fn get(pool: &SqlitePool, id: i64) -> Result<Character, sqlx::Error> {
    sqlx::query_as!(Character, "SELECT * FROM character WHERE id = ?", id)
        .fetch_one(pool)
        .await
}
