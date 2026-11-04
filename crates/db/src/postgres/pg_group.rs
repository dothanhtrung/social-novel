use crate::db_group::{Group, GroupMember};
use sqlx::PgPool;

pub(crate) async fn search(pool: &PgPool, search: &str, member: Option<i64>) -> Result<Vec<Group>, sqlx::Error> {
    sqlx::query_as!(
        Group,
        r#"SELECT id, name, description FROM groups
            WHERE name ILIKE '%' || $1 || '%'
              AND (CASE WHEN $2::INT8 IS NOT NULL THEN EXISTS (
                    SELECT 1 FROM group_member
                    WHERE group_member."group" = groups.id AND group_member.character = $2
                  ) ELSE TRUE END)
            ORDER BY name"#,
        search,
        member,
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn members(pool: &PgPool, group_id: i64) -> Result<Vec<GroupMember>, sqlx::Error> {
    sqlx::query_as!(
        GroupMember,
        r#"SELECT character.id AS "id!", character.username AS "username!", character.name AS "name!"
            FROM group_member
            INNER JOIN character ON character.id = group_member.character
            WHERE group_member."group" = $1
            ORDER BY character.name"#,
        group_id,
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn join(pool: &PgPool, character_id: i64, group_id: i64) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(
        r#"INSERT INTO group_member (character, "group", role) VALUES ($1, $2, 0)
           ON CONFLICT (character, "group") DO NOTHING"#,
        character_id,
        group_id,
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(count)
}

pub(crate) async fn leave(pool: &PgPool, character_id: i64, group_id: i64) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(
        r#"DELETE FROM group_member WHERE character = $1 AND "group" = $2"#,
        character_id,
        group_id,
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(count)
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

pub(crate) async fn update(pool: &PgPool, group: &Group) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE groups SET name = $1, description = $2 WHERE id = $3",
        group.name,
        group.description,
        group.id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn delete_by_id(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let count = sqlx::query_scalar!("DELETE FROM groups WHERE id = $1", id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}
