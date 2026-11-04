use crate::db_friend::Friend;
use sqlx::PgPool;

pub(crate) async fn list(pool: &PgPool, character_id: i64) -> Result<Vec<Friend>, sqlx::Error> {
    sqlx::query_as!(
        Friend,
        r#"WITH pairs AS (
            SELECT CASE WHEN "character" = $1 THEN friend ELSE "character" END AS friend_id
            FROM friend
            WHERE "character" = $1 OR friend = $1
        ),
        pair_room AS (
            SELECT DISTINCT ON (pairs.friend_id)
                pairs.friend_id,
                chat_room.id AS room_id,
                COALESCE(latest.content, '') AS last_message,
                COALESCE(latest.author, 0) AS last_author,
                COALESCE(to_char(latest.updated_at, 'YYYY-MM-DD HH24:MI:SS'), '') AS last_at,
                COALESCE(latest.updated_at, chat_room.updated_at) AS sort_at
            FROM pairs
            INNER JOIN chat_room_member AS me ON me.character = $1
            INNER JOIN chat_room_member AS them
                ON them.room = me.room AND them.character = pairs.friend_id
            INNER JOIN chat_room ON chat_room.id = me.room
            LEFT JOIN LATERAL (
                SELECT content, author, updated_at
                FROM post
                WHERE post.room = chat_room.id AND post.parent IS NULL
                ORDER BY updated_at DESC
                LIMIT 1
            ) latest ON TRUE
            WHERE (
                SELECT COUNT(DISTINCT m.character)
                FROM chat_room_member AS m
                WHERE m.room = chat_room.id
            ) = 2
            ORDER BY pairs.friend_id, latest.updated_at DESC NULLS LAST, chat_room.updated_at DESC
        )
        SELECT character.id AS "id!",
            character.username AS "username!",
            character.name AS "name!",
            COALESCE(pair_room.room_id, 0) AS "room_id!",
            COALESCE(pair_room.last_message, '') AS "last_message!",
            COALESCE(pair_room.last_author, 0) AS "last_author!",
            COALESCE(pair_room.last_at, '') AS "last_at!"
        FROM pairs
        INNER JOIN character ON character.id = pairs.friend_id
        LEFT JOIN pair_room ON pair_room.friend_id = pairs.friend_id
        ORDER BY pair_room.sort_at DESC NULLS LAST, character.name"#,
        character_id
    )
    .fetch_all(pool)
    .await
}

pub(crate) async fn insert(pool: &PgPool, character_id: i64, friend_id: i64) -> Result<i64, sqlx::Error> {
    let id = sqlx::query_scalar!(
        r#"INSERT INTO friend ("character", friend) VALUES ($1, $2) RETURNING id"#,
        character_id,
        friend_id
    )
    .fetch_one(pool)
    .await?;
    Ok(id)
}

pub(crate) async fn delete_pair(pool: &PgPool, character_id: i64, friend_id: i64) -> Result<u64, sqlx::Error> {
    let count = sqlx::query!(
        r#"DELETE FROM friend
           WHERE ("character" = $1 AND friend = $2) OR ("character" = $2 AND friend = $1)"#,
        character_id,
        friend_id
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(count)
}
