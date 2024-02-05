use chrono::{DateTime, Utc};
pub struct Favorite {
    name: String,
    note: i32,
    lastViewed: DateTime<Utc>,
    imgPath: String,
    sites: Vec<String>,
}

impl Favorite {
    pub fn new(name: String, note: i32, lastViewed: DateTime<Utc>, imgPath: String, sites: Vec<String>) -> Self {
        Self {
            name,
            note,
            lastViewed,
            imgPath,
            sites,
        }
    }
    pub fn setSites(&mut self, sites: Vec<String>) {
        self.sites = sites;
    }
    pub fn getSites(&self) -> Vec<String> {
        self.sites.clone()
    }

    pub fn setNote(&mut self, note: i32) {
        self.note = note;
    }
    pub fn getNote(&self) -> i32 {
        self.note
    }

}
