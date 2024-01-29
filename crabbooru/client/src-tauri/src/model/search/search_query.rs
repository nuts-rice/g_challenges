use serde_json::*;
use crate::model::{Profile};
#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct SearchQuery {
    pub tags : Vec<String>,
    pub gallery: Vec<Image>,
    pub urls: Hashmap<String, String>,

}

impl SearchQuery {
    pub fn new(tags: Vec<String>, gallery: Vec<Image>, urls: Hashmap<String, String>) -> Self {
        Self {
            tags,
            gallery,
            urls,
        }
    }

    pub fn read(json: &str, profile: Profile) -> Result<()> {

        let query: SearchQuery = serde_json::from_str(json)?;
        let tags = query["tags"];
        if query.contains_key("tags") {
            let tags = query["tags"].as_array().unwrap();
            tags.iter().for_each(|tag| {
                tags.push(tag.as_str().unwrap().to_string());
            });
        }
        if query.contains_key("gallery") {
            let sites: Hashmap<String, Site> = profile.
            gallery.iter().for_each(|image| {
                gallery.push(image.as_str().unwrap().to_string());
            });
        }
        Ok(query)        
    }

    pub fn write(value: &str, profile: Profile) -> Result<()> {
        let query = serde_json::to_string(value)?;
        Ok(query)

    }
}

impl fmt::Display for SearchQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //if (!self.gallery.is_none()) { write!(f, "Gallery: {:?}", self.gallery) }")})
        write!(f, "tags: {:?}, urls : {:?}", self.tags. self.urls) )
    }
}