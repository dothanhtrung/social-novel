use crate::{CommonMessage, delete_files, save_file};
use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use my_config::{ChatSetupVars, ConfigData, LLMConfig};
use my_db::db_character::{self, Character};
use my_db::db_media::Media;
use my_db::db_post::{Post, SearchPostCondition, string_to_offsetdatetime};
use my_db::{db_media, db_post, db_room};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use std::cmp::max;
use std::path::{Path, PathBuf};
use tracing::{debug, error};
use web_misc::db::DBPool;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/post").service(get).service(update).service(delete));
}

#[derive(Serialize)]
struct PostResponse {
    posts: Vec<Post>,
    err: String,
}

#[derive(Deserialize)]
struct PostQuery {
    id: Option<i64>,
    page: Option<i64>,
    count: Option<i64>,
    author: Option<i64>,
    parent: Option<i64>,
    group: Option<i64>, // TODO: query by multiple groups, authors
    room: Option<i64>,
    /// When true, only root posts (parent is null) are returned.
    roots: Option<bool>,
}

#[derive(MultipartForm, Debug)]
struct PostForm {
    id: Text<i64>,
    content: Text<String>,
    media: Vec<TempFile>,
    remove_media: Vec<Text<i64>>,
    author: Text<i64>,
    parent: Text<i64>,
    liked: Text<i32>,
    haha: Text<i32>,
    loved: Text<i32>,
    surprised: Text<i32>,
    sad: Text<i32>,
    feeling: Text<String>,
    is_with: Text<String>,
    liked_by: Text<String>,
    group: Text<i64>,
    room: Text<i64>,
    updated_at: Option<Text<String>>,
}

#[get("")]
async fn get(
    config_data: web::Data<ConfigData>,
    dbpool: web::Data<DBPool>,
    query_params: Query<PostQuery>,
) -> impl Responder {
    let config = config_data.config.read().await;
    let mut ret = Vec::new();
    let mut err = String::new();
    let page = max(1, query_params.page.unwrap_or(1)) - 1;
    let limit = max(0, query_params.count.unwrap_or(config.api.per_page as i64));
    let offset = page * limit;
    let mut cond = SearchPostCondition::default();
    cond.authors = if query_params.author.is_some() {
        Some(vec![query_params.author.unwrap()])
    } else {
        None
    };
    cond.groups = if query_params.group.is_some() {
        Some(vec![query_params.group.unwrap()])
    } else {
        None
    };
    cond.rooms = if query_params.room.is_some() {
        Some(vec![query_params.room.unwrap()])
    } else {
        None
    };
    cond.roots_only = query_params.roots.unwrap_or(false);

    if let Some(id) = query_params.id {
        match db_post::get_by_id(&dbpool, id).await {
            Ok(post) => ret = vec![post],
            Err(e) => err = e.to_string(),
        }
    } else if let Some(parent) = query_params.parent {
        match db_post::get_by_parent(&dbpool, parent).await {
            Ok(posts) => ret = posts,
            Err(e) => err = e.to_string(),
        }
    } else {
        match db_post::get_all(&dbpool, limit, offset, &cond).await {
            Ok(posts) => ret = posts,
            Err(e) => err = e.to_string(),
        }
    }

    web::Json(PostResponse { posts: ret, err })
}

/// Walk parents until a post that belongs to a group is found.
async fn inherit_group(dbpool: &DBPool, mut parent: Option<i64>) -> Option<i64> {
    for _ in 0..32 {
        let Some(parent_id) = parent.filter(|id| *id > 0) else {
            return None;
        };
        let Ok(post) = db_post::get_by_id(dbpool, parent_id).await else {
            return None;
        };
        if let Some(group_id) = post.group.filter(|id| *id > 0) {
            return Some(group_id);
        }
        parent = post.parent;
    }
    None
}

#[post("")]
async fn update(
    dbpool: web::Data<DBPool>,
    config_data: web::Data<ConfigData>,
    MultipartForm(data): MultipartForm<PostForm>,
) -> impl Responder {
    let mut msg = String::new();
    let mut err = String::new();
    let config = config_data.config.read().await;
    debug!("Received form: {:?}", data);
    let form_parent = data.parent.into_inner();
    let parent = if form_parent > 0 { Some(form_parent) } else { None };
    let is_new = data.id.0 == 0;
    let mut group = if data.group.0 == 0 { None } else { Some(data.group.0) };
    // A comment with no group inherits the group of the post (or ancestor) it replies to.
    if is_new && group.is_none() {
        group = inherit_group(&dbpool, parent).await;
    }
    let room = if data.room.0 == 0 { None } else { Some(data.room.0) };
    let updated_at = if let Some(timestamp) = data.updated_at {
        string_to_offsetdatetime(timestamp.into_inner().as_str()).unwrap_or(OffsetDateTime::now_utc())
    } else {
        OffsetDateTime::now_utc()
    };
    let post = Post {
        id: data.id.into_inner(),
        content: data.content.into_inner(),
        author: data.author.into_inner(),
        parent,
        liked: data.liked.into_inner(),
        haha: data.haha.into_inner(),
        loved: data.loved.into_inner(),
        surprised: data.surprised.into_inner(),
        sad: data.sad.into_inner(),
        feeling: data.feeling.into_inner(),
        is_with: data.is_with.into_inner(),
        liked_by: data.liked_by.into_inner(),
        created_at: OffsetDateTime::now_utc(),
        updated_at,
        group,
        room,
    };

    let mut post_id = post.id;
    if post.id > 0 {
        match db_post::update(&dbpool, &post).await {
            Ok(_) => msg = "Success".to_string(),
            Err(e) => {
                err = e.to_string();
                return web::Json(CommonMessage::new(msg, err));
            }
        }
    } else {
        match db_post::insert(&dbpool, &post).await {
            Ok(id) => {
                post_id = id;
                msg = "Success".to_string();
            }
            Err(e) => {
                err = e.to_string();
                return web::Json(CommonMessage::new(msg, err));
            }
        }
    }

    if let Some(room) = room
        && let Err(e) = my_db::db_room::touch(&dbpool, room).await
    {
        error!("Failed to touch room {}: {}", room, e);
    }

    let mut remove_media: Vec<i64> = data.remove_media.into_iter().map(Text::into_inner).collect();
    let data_dir = PathBuf::from(&config.data_dir);
    for media in data.media {
        // Browsers send an empty file part when the file input is unused. Skip it so
        // a blank NA attachment cannot sort ahead of real media and hide it.
        if media.size == 0 || media.file_name.as_deref().unwrap_or("").is_empty() {
            continue;
        }
        match save_file(&data_dir, media, "").await {
            Ok((blake3, file_name, file_type)) => {
                let media = Media {
                    id: 0,
                    url: file_name,
                    blake3,
                    file_type,
                };
                match db_media::attach(&dbpool, post_id, &media).await {
                    Ok(id) => remove_media.retain(|media_id| *media_id != id),
                    Err(e) => err = e.to_string(),
                }
            }
            Err(e) => err = e.to_string(),
        }
    }

    if err.is_empty() && !is_new && !remove_media.is_empty() {
        match db_media::unlink_media(&dbpool, post_id, &remove_media).await {
            Ok(urls) => delete_files(&data_dir, &urls).await,
            Err(e) => err = e.to_string(),
        }
    }

    // A new root chat message can ask the LLM to answer as the other member.
    // The user's message is already stored; a failed reply does not fail this request.
    if is_new && parent.is_none() && !post.content.trim().is_empty() {
        if let Some(room_id) = room {
            let llm = config.llm.clone();
            drop(config);
            maybe_llm_reply(&dbpool, &llm, room_id, post.author, post_id, &post.content).await;
        }
    }

    // id lets the page insert the new comment without reloading.
    web::Json(CommonMessage::new(msg, err).with_id(post_id))
}

/// When the room has LLM replies on, post a reply as the other member.
async fn maybe_llm_reply(
    dbpool: &DBPool,
    llm: &LLMConfig,
    room_id: i64,
    sender_id: i64,
    current_post_id: i64,
    current_message: &str,
) {
    if llm.url.trim().is_empty() {
        return;
    }

    let room = match db_room::get(dbpool, room_id).await {
        Ok(room) => room,
        Err(e) => {
            error!("LLM reply: load room {room_id}: {e}");
            return;
        }
    };
    if !room.llm_enabled {
        return;
    }
    let recent_n = room.llm_recent.clamp(1, 50);

    let members = match db_character::search(
        dbpool,
        &db_character::CharacterCond {
            room: Some(room_id),
            search: None,
        },
    )
    .await
    {
        Ok(members) => members,
        Err(e) => {
            error!("LLM reply: load room {room_id} members: {e}");
            return;
        }
    };
    let Some(receiver) = members.iter().find(|member| member.id != sender_id).cloned() else {
        return;
    };
    let sender = match members.iter().find(|member| member.id == sender_id).cloned() {
        Some(sender) => sender,
        None => match db_character::get(dbpool, sender_id).await {
            Ok(sender) => sender,
            Err(e) => {
                error!("LLM reply: load sender {sender_id}: {e}");
                return;
            }
        },
    };

    let mut cond = SearchPostCondition::default();
    cond.rooms = Some(vec![room_id]);
    cond.roots_only = true;
    let recent_messages = match db_post::get_all(dbpool, i64::from(recent_n) + 1, 0, &cond).await {
        Ok(posts) => format_recent_messages(&posts, current_post_id, recent_n, &members),
        Err(e) => {
            error!("LLM reply: load recent messages: {e}");
            return;
        }
    };

    let prompt = llm.chat_setup_message(&ChatSetupVars {
        sender_name: &sender.name,
        sender_username: &sender.username,
        sender_back_story: &sender.back_story,
        receiver_name: &receiver.name,
        receiver_username: &receiver.username,
        receiver_back_story: &receiver.back_story,
        recent_messages: &recent_messages,
        current_message: current_message.trim(),
    });

    let reply = match crate::llm::chat_completion(&llm.url, &prompt).await {
        Ok(reply) => reply,
        Err(e) => {
            error!("LLM reply failed: {e}");
            return;
        }
    };

    let now = OffsetDateTime::now_utc();
    let reply_post = Post {
        id: 0,
        content: reply,
        author: receiver.id,
        parent: None,
        liked_by: String::new(),
        liked: 0,
        haha: 0,
        loved: 0,
        surprised: 0,
        sad: 0,
        feeling: String::new(),
        is_with: String::new(),
        created_at: now,
        updated_at: now,
        group: None,
        room: Some(room_id),
    };
    // insert, not update: update is the HTTP handler and would call this again.
    if let Err(e) = db_post::insert(dbpool, &reply_post).await {
        error!("LLM reply: insert failed: {e}");
        return;
    }
    if let Err(e) = db_room::touch(dbpool, room_id).await {
        error!("LLM reply: touch room {room_id}: {e}");
    }
}

fn format_recent_messages(posts: &[Post], current_post_id: i64, recent_n: i32, members: &[Character]) -> String {
    let mut recent: Vec<&Post> = posts.iter().filter(|post| post.id != current_post_id).collect();
    let keep = usize::try_from(recent_n).unwrap_or(0);
    recent.truncate(keep);
    recent.reverse();
    recent
        .into_iter()
        .map(|post| {
            let name = members
                .iter()
                .find(|member| member.id == post.author)
                .map(|member| member.name.as_str())
                .unwrap_or("Unknown");
            format!("{name}: {}", post.content)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[get("delete/{id}")]
async fn delete(dbpool: web::Data<DBPool>, config_data: web::Data<ConfigData>, id: web::Path<i64>) -> impl Responder {
    let mut msg = String::new();
    let mut err = String::new();
    let config = config_data.config.read().await;
    let post_id = id.into_inner();

    // Drop this post's links. Files still used by another post are kept.
    match db_media::unlink_posts(&dbpool, &[post_id]).await {
        Ok(urls) => {
            match db_post::delete_by_id(&dbpool, post_id).await {
                Ok(_) => {
                    msg = "Success".to_string();
                    delete_files(Path::new(&config.data_dir), &urls).await;
                }
                Err(e) => err = e.to_string(),
            }
        }
        Err(e) => err = e.to_string(),
    }
    web::Json(CommonMessage::new(msg, err))
}
