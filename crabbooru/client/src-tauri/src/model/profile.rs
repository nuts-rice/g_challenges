use crate::{Site, Favorite};

pub struct Profile {
    pub name: String,
    pub sites: Vec<Site>,
    pub favorites: Vec<Favorite>,
    pub selected: usize,
}

impl Profile {
    pub fn new(name: String, sites: Vec<Site>, favorites: Vec<Favorite>, selected: usize) -> Self {
        Self {
            name,
            sites,
            favorites,
            selected,
        }
    }
}