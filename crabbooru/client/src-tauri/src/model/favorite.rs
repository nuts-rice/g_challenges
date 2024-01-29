use chrono::{DateTime, Utc};
use crate::Site;
pub struct Favorite {
    name: String,
    note: i32,
    lastViewed: DateTime<Utc>,
    imgPath: String,
    sites: Vec<Site>,
}

impl Favorite {
    pub fn new(name: String, note: i32, lastViewed: DateTime<Utc>, imgPath: String, sites: Vec<Site>) -> Self {
        Self {
            name,
            note,
            lastViewed,
            imgPath,
            sites,
        }
    }
    pub fn setSites(&mut self, sites: Vec<Site>) {
        self.sites = sites;
    }
    pub fn getSites(&self) -> Vec<Site> {
        self.sites.clone()
    }

    pub fn setNote(&mut self, note: i32) {
        self.note = note;
    }
    pub fn getNote(&self) -> i32 {
        self.note
    }

}