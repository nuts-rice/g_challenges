use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DanbooruPost {
    pub id: i32,

}

impl std::fmt::Display for DanbooruPost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lower_case = format!("{:?}" , self).to_lowercase();
        write!(f, "DanbooruPost: id={}", self.id)
    }
}

