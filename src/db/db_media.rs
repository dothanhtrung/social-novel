use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(i64)]
pub enum MediaType {
    Image = 0,
    Video = 1,
    Audio = 2,
}

impl From<i64> for MediaType {
    fn from(value: i64) -> Self {
        match value {
            0 => MediaType::Image,
            1 => MediaType::Video,
            2 => MediaType::Audio,
            _ => MediaType::Image,
        }
    }   
}

#[derive(Serialize, Deserialize)]
pub struct  Media {
    #[serde(default)]
    pub id: i64,
    pub url: String,
    pub file_type: MediaType,
    pub post: i64,
}

