use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DanbooruPost {
    pub id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub uploader_id: u32,
    pub approver_id: Option<u32>,
    pub tag_string: String,
    pub tag_string_general: String,
    pub tag_string_artist: String,
    pub tag_string_copyright: String,
    pub tag_string_character: String,
    pub tag_string_meta: String,
    //pub rating: Option<DanbooruRating>,
    pub parent_id: Option<u32>,
    pub pixiv_id: Option<u32>,
    pub source: String,
    pub md5: Option<String>,
    pub file_url: Option<String>,
    pub large_file_url: Option<String>,
    pub preview_file_url: Option<String>,
    pub file_ext: String,
    pub file_size: u32,
    pub image_width: u32,
    pub image_height: u32,
    pub score: i32,
    pub up_score: i32,
    pub down_score: i32,
    pub fav_count: u32,
    pub tag_count_general: u32,
    pub tag_count_artist: u32,
    pub tag_count_copyright: u32,
    pub tag_count_character: u32,
    pub tag_count_meta: u32,
    pub last_comment_bumped_at: Option<String>,
    pub last_noted_at: Option<String>,
    pub has_large: bool,
    pub has_children: bool,
    pub has_visible_children: bool,
    pub has_active_children: bool,
    pub is_banned: bool,
    pub is_deleted: bool,
    pub is_flagged: bool,
    pub is_pending: bool,
    pub bit_flags: u32,

}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DanbooruRating {
    #[serde(rename = "g")]
    General,
    #[serde(rename = "q")]  
    Questionable,
    #[serde(rename = "e")]  
    Explicit,
    #[serde(rename = "s")]
    Sensitive
}
impl From<DanbooruRating> for String {
    fn from(rating: DanbooruRating) -> String {
        rating.to_string()
    }
}

impl std::fmt::Display for DanbooruRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lower_case = format!("{:?}" , self).to_lowercase();
        write!(f, "DanbooruRating: {}", lower_case)
    }
}

impl std::fmt::Display for DanbooruPost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lower_case = format!("{:?}" , self).to_lowercase();
        write!(f, "DanbooruPost: id={}", self.id)
    }
}

