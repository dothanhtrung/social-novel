//! Copyright (c) 2025 Trung Do <dothanhtrung@pm.me>.

pub mod db_ads;
pub mod db_character;
pub mod db_room;
pub mod db_group;
pub mod db_media;
pub mod db_post;
pub mod db_relation;
#[cfg(feature = "postgres")]
pub mod postgres;