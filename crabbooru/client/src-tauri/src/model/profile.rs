use crate::Favorite;

pub struct Profile {
    pub name: String,
    pub sites: Vec<String>,
    pub favorites: Vec<Favorite>,
    pub selected: usize,
}

impl Profile {
    pub fn new(
        name: String,
        sites: Vec<String>,
        favorites: Vec<Favorite>,
        selected: usize,
    ) -> Self {
        Self {
            name,
            sites,
            favorites,
            selected,
        }
    }
}
