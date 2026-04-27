use sqlx::PgPool;
use crate::db_group::Group;

pub(crate) async fn search(pool: &PgPool, search: &str) -> Result<Vec<Group>, sqlx::Error> {
    sqlx::query_as!(
        Group,
        r#"SELECT * FROM groups
            WHERE name ILIKE '%' || $1 || '%'
            ORDER BY name"#,
        search,
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn insert(pool: &PgPool, group: &Group) -> Result<i64, sqlx::Error> {
    let id = sqlx::query!(
        "INSERT INTO groups (name, description) VALUES ($1, $2) RETURNING id",
        group.name,
        group.description
    )
    .fetch_one(pool)
    .await?
    .id;
    Ok(id)
}

pub(crate) async fn delete_by_id(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let count = sqlx::query_scalar!("DELETE FROM groups WHERE id = $1", id)
        .execute(pool)
        .await?.rows_affected();
    Ok(count)
}
