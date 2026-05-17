use crate::db_room::{ChatRoom, RoomQuery, RoomMember};
use sqlx::PgPool;

pub(crate) async fn search(pool: &PgPool, query: &RoomQuery) -> Result<Vec<ChatRoom>, sqlx::Error> {
    let ids = query.id.as_deref();
    let members = query.member.as_deref();
    let limit = query.count.unwrap_or_default();
    let page = query.page.unwrap_or(1);
    let offset = (page - 1) * limit;

    sqlx::query_as!(
        ChatRoom,
        r#"SELECT chat_room.id as "id!", chat_room.name as "name!" FROM chat_room
        LEFT JOIN chat_room_member ON chat_room.id = chat_room_member.room
        WHERE
            (CASE WHEN $1::INT8[] IS NOT NULL THEN chat_room.id = ANY($1) ELSE TRUE END)
            AND (CASE WHEN $2::INT8[] IS NOT NULL THEN chat_room_member.character = ANY($2) ELSE TRUE END)
        ORDER BY updated_at DESC LIMIT $3 OFFSET $4
        "#,
        ids,
        members,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn delete_by_id(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(r#"DELETE FROM chat_room WHERE id = $1"#, id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}

pub(crate) async fn insert(pool: &PgPool, room_info: &ChatRoom) -> Result<i64, sqlx::Error> {
    let id = sqlx::query_scalar!(
        r#"INSERT INTO chat_room (name) VALUES ($1) RETURNING id"#,
        room_info.name
    )
    .fetch_one(pool)
    .await?;
    Ok(id)
}

pub(crate) async fn update(pool: &PgPool, room_info: &ChatRoom) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(
        r#"UPDATE chat_room SET name = $1 WHERE id = $2"#,
        room_info.name,
        room_info.id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(count)
}

pub(crate) async fn invite(pool: &PgPool, info: &RoomMember) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO chat_room_member (room, character)  VALUES ($1, $2)"#,
        info.room,
        info.member
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn kick(pool: &PgPool, info: &RoomMember) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"DELETE FROM chat_room_member WHERE room = $1 AND character = $2"#,
        info.room,
        info.member
    )
    .execute(pool)
    .await?;
    Ok(())
}
