use crate::{BooruSite};
use chrono::{DateTime, Utc};
pub struct Favorite {
    name: String,
    note: i32,
    lastViewed: DateTime<Utc>,
    imgPath: String,
    sites: Vec<BooruSite>,
}

impl Favorite {
    pub fn new(
        name: String,
        note: i32,
        lastViewed: DateTime<Utc>,
        imgPath: String,
        sites: Vec<BooruSite>,
    ) -> Self {
        Self {
            name,
            note,
            lastViewed,
            imgPath,
            sites,
        }
    }
    pub fn setSites(&mut self, sites: Vec<BooruSite>) {
        self.sites = sites;
    }
    pub fn getSites(&self) -> Vec<BooruSite> {
        self.sites.clone()
    }

    pub fn setNote(&mut self, note: i32) {
        self.note = note;
    }
    pub fn getNote(&self) -> i32 {
        self.note
    }

    async fn favorite_post(&self) {}
}
impl From<serde_json::Value> for Favorite {
    fn from(_value: serde_json::Value) -> Self {
        todo!()
    }
}

//TODOl: impl From<Post> for Favorite
// impl From<Post> for Favorite {
//         fn from(post: Post) -> Self {
//             Self {
//                 name: post.id.to_string(),
//                 note: 0,
//                 lastViewed: Utc::now(),
//                 imgPath: post.file_ext,
//                 sites: vec![post..clone()]
//             }
//         }
//     }
