use crate::db_relation::{Relation, RelationCond};
use sqlx::PgPool;

pub(crate) async fn search(pool: &PgPool, cond: &RelationCond) -> Result<Vec<Relation>, sqlx::Error> {
    sqlx::query_as!(
        Relation,
        r#"SELECT relation.id,
            relation.from_character,
            relation.to_character,
            relation.relation,
            from_c.name AS from_name,
            from_c.username AS from_username,
            to_c.name AS to_name,
            to_c.username AS to_username
        FROM relation
        INNER JOIN character AS from_c ON from_c.id = relation.from_character
        INNER JOIN character AS to_c ON to_c.id = relation.to_character
        WHERE (CASE WHEN $1::INT8 IS NOT NULL
            THEN relation.from_character = $1 OR relation.to_character = $1
            ELSE TRUE END)
        ORDER BY from_c.name, relation.relation, to_c.name"#,
        cond.character,
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn get(pool: &PgPool, id: i64) -> Result<Relation, sqlx::Error> {
    sqlx::query_as!(
        Relation,
        r#"SELECT relation.id,
            relation.from_character,
            relation.to_character,
            relation.relation,
            from_c.name AS from_name,
            from_c.username AS from_username,
            to_c.name AS to_name,
            to_c.username AS to_username
        FROM relation
        INNER JOIN character AS from_c ON from_c.id = relation.from_character
        INNER JOIN character AS to_c ON to_c.id = relation.to_character
        WHERE relation.id = $1"#,
        id
    )
    .fetch_one(pool)
    .await
}

pub(crate) async fn insert(pool: &PgPool, relation: &Relation) -> Result<i64, sqlx::Error> {
    let id = sqlx::query_scalar!(
        "INSERT INTO relation (from_character, to_character, relation) VALUES ($1, $2, $3) RETURNING id",
        relation.from_character,
        relation.to_character,
        relation.relation
    )
    .fetch_one(pool)
    .await?;
    Ok(id)
}

pub(crate) async fn update(pool: &PgPool, relation: &Relation) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(
        "UPDATE relation SET from_character = $1, to_character = $2, relation = $3 WHERE id = $4",
        relation.from_character,
        relation.to_character,
        relation.relation,
        relation.id
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(count)
}

pub(crate) async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let affected_rows = sqlx::query!("DELETE FROM relation WHERE id = $1", id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(affected_rows)
}
