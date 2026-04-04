use crate::db::db_chat::ChatRoom;
use sqlx::PgPool;

pub(crate) async fn get_by_id(pool: &PgPool, id: i64) -> Result<Vec<ChatRoom>, sqlx::Error> {
    sqlx::query_as!(ChatRoom, r#"SELECT * FROM chat_room WHERE id = $1"#, id)
        .fetch_all(pool)
        .await
}

pub(crate) async fn get_by_member(pool: &PgPool, member: i64) -> Result<Vec<ChatRoom>, sqlx::Error> {
    sqlx::query_as!(
        ChatRoom,
        r#"SELECT chat_room.id as id, chat_room.name as name FROM chat_room
            LEFT JOIN chat_room_member ON chat_room.id = chat_room_member.room
            WHERE chat_room_member.character = $1"#,
        member
    )
    .fetch_all(pool)
    .await
}
