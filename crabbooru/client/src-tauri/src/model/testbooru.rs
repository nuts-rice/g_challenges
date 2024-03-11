use serde::{Deserialize, Serialize};
use crate::{model::img_factory::*};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestbooruPost {
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
    pub rating: Option<TestbooruRating>,
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
pub enum TestbooruRating {
    #[serde(rename = "g")]
    General,
    #[serde(rename = "q")]
    Questionable,
    #[serde(rename = "e")]
    Explicit,
    #[serde(rename = "s")]
    Sensitive,
}

impl From<TestbooruRating> for String {
    fn from(rating: TestbooruRating) -> String {
        rating.to_string()
    }
}

impl std::fmt::Display for TestbooruRating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lower_case = format!("{:?}", self).to_lowercase();
        write!(f, "TestbooruRating: {}", lower_case)
    }
}
impl std::fmt::Display for TestbooruPost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _lower_case = format!("{:?}", self).to_lowercase();
        write!(f, "TestbooruPost: id={}", self.id)
    }
}

impl Image for TestbooruPost {}
    // fn get_id(&self) -> i32 {
    //     self.id
    // }
    // fn get_url(&self) -> String {
    //     self.file_url.clone().unwrap_or("".to_string())
    // }
    // fn get_preview_url(&self) -> String {
    //     self.preview_file_url.clone().unwrap_or("".to_string())
    // }
    // fn get_tags(&self) -> String {
    //     self.tag_string.clone()
    // }
    // fn get_rating(&self) -> String {
    //     self.rating.clone().unwrap_or(TestbooruRating::General).to_string()
    // }
    // fn get_source(&self) -> String {
    //     self.source.clone()
    // }
    // fn get_width(&self) -> u32 {
    //     self.image_width
    // }
    // fn get_height(&self) -> u32 {
    //     self.image_height
    // }
    // fn get_score(&self) -> i32 {
    //     self.score
    // }
    // fn get_up_score(&self) -> i32 {
    //     self.up_score
    // }
    // fn get_down_score(&self) -> i32 {
    //     self.down_score
    // }
    // fn get_fav_count(&self) -> u32 {
    //     self.fav_count
    // }
    // fn get_tag_count(&self) -> u32 {
    //     self.tag_count_general
    // }
    // fn get_created_at(&self) -> String {
    //     self.created_at.clone()
    // }
    // fn get_updated_at(&self) -> String {
    //     self.updated_at.clone()
    // }
    // fn get_file_size(&self) -> u32 {
    //     self.file_size
    // }
    // fn get_file_ext(&self) -> String {
    //     self.file_ext.clone()
    // }
    // fn get_parent_id(&self) -> Option<u32> {
    //     self.parent_id
    // }
    // fn get_pixiv_id(&self) -> Option<u32> {
    //     self.pixiv_id
    // }
    // fn get_md5(&self) -> Option<String> {
    //     self.md5.clone()
    // }
    // fn get_large_file_url(&self) -> Option<String> {
    //     self.large_file_url.clone()
    // }
    // fn get_last_comment_bumped_at(&self) -> Option<String> {
    //     self.last_comment_bumped_at.clone()
    // }
    // fn get_last_noted_at(&self) -> Option<String> {
    //     self.last_noted_at.clone()
    // }
    // fn get_bit_flags(&self) -> u32 {
       
