use crate::db_post::{Post, SearchPostCondition};
use sqlx::PgPool;

pub(crate) async fn insert(pool: &PgPool, post: &Post) -> Result<i64, sqlx::Error> {
    let id = sqlx::query!(
        r#"INSERT INTO post
            (content, author, parent, liked, haha, loved, surprised, sad, feeling, is_with, liked_by, "group", room)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
            RETURNING id"#,
        post.content,
        post.author,
        post.parent,
        post.liked,
        post.haha,
        post.loved,
        post.surprised,
        post.sad,
        post.feeling,
        post.is_with,
        post.liked_by,
        post.group,
        post.room,
    )
        .fetch_one(pool)
        .await?
        .id;
    Ok(id)
}

pub(crate) async fn get_by_id(pool: &PgPool, id: i64) -> Result<Post, sqlx::Error> {
    sqlx::query_as!(Post, r#"SELECT * FROM post WHERE id = $1"#, id)
        .fetch_one(pool)
        .await
}

pub(crate) async fn get_all(
    pool: &PgPool,
    limit: i64,
    offset: i64,
    cond: &SearchPostCondition,
) -> Result<Vec<Post>, sqlx::Error> {
    let author_ids = cond.authors.as_deref();
    let group_ids = cond.groups.as_deref();
    let room_ids = cond.rooms.as_deref();
    sqlx::query_as!(
        Post,
        r#"SELECT * FROM post
        WHERE
            (CASE WHEN $1::INT8[] IS NOT NULL THEN author = ANY($1) ELSE TRUE END)
            AND (CASE WHEN $2::INT8[] IS NOT NULL THEN "group" = ANY($2) ELSE TRUE END)
            AND (CASE WHEN $3::INT8[] IS NOT NULL THEN room = ANY($3) ELSE TRUE END)
        ORDER BY updated_at DESC LIMIT $4 OFFSET $5"#,
        author_ids,
        group_ids,
        room_ids,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn get_by_parent(pool: &PgPool, parent_id: i64) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as!(
        Post,
        r#"SELECT * FROM post WHERE parent = $1 ORDER BY updated_at"#,
        parent_id
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn update(pool: &PgPool, post: &Post) -> Result<u64, sqlx::Error> {
    // TODO: Update updated_at
    let count = sqlx::query!(
        r#"UPDATE post SET
                content = $1,
                author = $2,
                parent = $3,
                liked = $4,
                haha = $5,
                loved = $6,
                surprised = $7,
                sad = $8,
                feeling = $9,
                is_with = $10,
                liked_by = $11,
                "group" = $12,
                room = $13,
                updated_at = $14
            WHERE id = $15"#,
        post.content,
        post.author,
        post.parent,
        post.liked,
        post.haha,
        post.loved,
        post.surprised,
        post.sad,
        post.feeling,
        post.is_with,
        post.liked_by,
        post.group,
        post.room,
        post.updated_at,
        post.id
    ).execute(pool).await?.rows_affected();

    Ok(count)
}

pub(crate) async fn delete_by_id(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!("DELETE FROM post WHERE id = $1", id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}

pub(crate) async fn delete_by_group(pool: &PgPool, group_id: i64) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(r#"DELETE FROM post WHERE "group" = $1"#, group_id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}

pub(crate) async fn delete_by_room(pool: &PgPool, room_id: i64) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(r#"DELETE FROM post WHERE room = $1"#, room_id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}
