use crate::db_ads::{Ads, AdsCond};
use sqlx::PgPool;

pub(crate) async fn search(pool: &PgPool, cond: &AdsCond) -> Result<Vec<Ads>, sqlx::Error> {
    sqlx::query_as!(
        Ads,
        r#"SELECT id, description, active AS "active!", media
            FROM ads
            WHERE (CASE WHEN $1::BOOL IS NOT NULL THEN active = $1 ELSE TRUE END)
            ORDER BY id DESC"#,
        cond.active,
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn get(pool: &PgPool, id: i64) -> Result<Ads, sqlx::Error> {
    sqlx::query_as!(
        Ads,
        r#"SELECT id, description, active AS "active!", media FROM ads WHERE id = $1"#,
        id
    )
    .fetch_one(pool)
    .await
}

pub(crate) async fn insert(pool: &PgPool, ads: &Ads) -> Result<i64, sqlx::Error> {
    let id = sqlx::query_scalar!(
        "INSERT INTO ads (description, active, media) VALUES ($1, $2, $3) RETURNING id",
        ads.description,
        ads.active,
        ads.media
    )
    .fetch_one(pool)
    .await?;
    Ok(id)
}

pub(crate) async fn update(pool: &PgPool, ads: &Ads) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(
        "UPDATE ads SET description = $1, active = $2, media = $3, updated_at = now() WHERE id = $4",
        ads.description,
        ads.active,
        ads.media,
        ads.id
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(count)
}

pub(crate) async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let affected_rows = sqlx::query!("DELETE FROM ads WHERE id = $1", id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(affected_rows)
}
