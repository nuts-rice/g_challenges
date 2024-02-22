use serde::Deserialize;
use std::fmt;
#[derive(Debug, Clone, Deserialize)]
pub struct SafebooruPost {
    pub id: i32,
    pub score: Option<u32>,
    pub height: u32,
    pub width: u32,
    pub hash: String,
    pub tags: String,
    pub image: String,
    pub change: u32,
    pub rating: SafebooruRating,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SafebooruRating {
    Safe,
    General,
    Questionable,
    Explicit,
}

impl From<SafebooruRating> for String {
    fn from(rating: SafebooruRating) -> String {
        rating.to_string()
    }
}

impl fmt::Display for SafebooruRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tag = format!("{:?}", self).to_lowercase();
        write!(f, "{tag}")
    }
}
