use crate::db_media::Media;
use sqlx::PgPool;

pub(crate) async fn attach(pool: &PgPool, post_id: i64, media: &Media) -> Result<i64, sqlx::Error> {
    let file_type = media.file_type as i16;
    let mut tx = pool.begin().await?;
    let id = sqlx::query_scalar!(
        r#"INSERT INTO media (url, file_type, blake3)
            VALUES ($1, $2, $3)
            ON CONFLICT (url) DO UPDATE SET url = EXCLUDED.url
            RETURNING id"#,
        media.url,
        file_type,
        media.blake3
    )
    .fetch_one(&mut *tx)
    .await?;
    sqlx::query!(
        r#"INSERT INTO post_media (post_id, media_id) VALUES ($1, $2)
            ON CONFLICT (post_id, media_id) DO NOTHING"#,
        post_id,
        id
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(id)
}

pub(crate) async fn get_by_post(pool: &PgPool, post: i64) -> Result<Vec<Media>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT m.id, m.url, m.file_type, m.blake3
            FROM media m
            INNER JOIN post_media pm ON pm.media_id = m.id
            WHERE pm.post_id = $1
            ORDER BY m.id"#,
        post
    )
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| Media {
            id: row.id,
            url: row.url,
            file_type: row.file_type.into(),
            blake3: row.blake3,
        })
        .collect())
}

/// Drop links for these posts. Media still used by another post is kept.
/// Returns urls of media rows that no longer belong to any post.
pub(crate) async fn unlink_posts(pool: &PgPool, post_ids: &[i64]) -> Result<Vec<String>, sqlx::Error> {
    if post_ids.is_empty() {
        return Ok(Vec::new());
    }
    let urls = sqlx::query_scalar!(
        r#"WITH removed AS (
                DELETE FROM post_media WHERE post_id = ANY($1) RETURNING media_id
            )
            DELETE FROM media m
            WHERE m.id IN (SELECT media_id FROM removed)
              AND NOT EXISTS (
                    SELECT 1 FROM post_media pm
                    WHERE pm.media_id = m.id AND NOT (pm.post_id = ANY($1))
              )
            RETURNING m.url"#,
        post_ids
    )
    .fetch_all(pool)
    .await?;
    Ok(urls)
}

pub(crate) async fn unlink_media(pool: &PgPool, post_id: i64, media_ids: &[i64]) -> Result<Vec<String>, sqlx::Error> {
    let urls = sqlx::query_scalar!(
        r#"WITH removed AS (
                DELETE FROM post_media
                WHERE post_id = $1 AND media_id = ANY($2)
                RETURNING media_id
            )
            DELETE FROM media m
            WHERE m.id IN (SELECT media_id FROM removed)
              AND NOT EXISTS (
                    SELECT 1 FROM post_media pm
                    WHERE pm.media_id = m.id AND pm.post_id <> $1
              )
            RETURNING m.url"#,
        post_id,
        media_ids
    )
    .fetch_all(pool)
    .await?;
    Ok(urls)
}

pub(crate) async fn unlink_group(pool: &PgPool, group_id: i64) -> Result<Vec<String>, sqlx::Error> {
    let urls = sqlx::query_scalar!(
        r#"WITH doomed AS (
                SELECT id FROM post WHERE "group" = $1
            ),
            removed AS (
                DELETE FROM post_media
                WHERE post_id IN (SELECT id FROM doomed)
                RETURNING media_id
            )
            DELETE FROM media m
            WHERE m.id IN (SELECT media_id FROM removed)
              AND NOT EXISTS (
                    SELECT 1 FROM post_media pm
                    WHERE pm.media_id = m.id
                      AND pm.post_id NOT IN (SELECT id FROM doomed)
              )
            RETURNING m.url"#,
        group_id
    )
    .fetch_all(pool)
    .await?;
    Ok(urls)
}

pub(crate) async fn unlink_room(pool: &PgPool, room_id: i64) -> Result<Vec<String>, sqlx::Error> {
    let urls = sqlx::query_scalar!(
        r#"WITH doomed AS (
                SELECT id FROM post WHERE room = $1
            ),
            removed AS (
                DELETE FROM post_media
                WHERE post_id IN (SELECT id FROM doomed)
                RETURNING media_id
            )
            DELETE FROM media m
            WHERE m.id IN (SELECT media_id FROM removed)
              AND NOT EXISTS (
                    SELECT 1 FROM post_media pm
                    WHERE pm.media_id = m.id
                      AND pm.post_id NOT IN (SELECT id FROM doomed)
              )
            RETURNING m.url"#,
        room_id
    )
    .fetch_all(pool)
    .await?;
    Ok(urls)
}
