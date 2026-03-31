use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ads {
    pub id: i64,
    pub description: String,
    pub active: bool,
}