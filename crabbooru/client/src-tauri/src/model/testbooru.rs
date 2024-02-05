use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestbooruPost {
    pub id: i32,

}

impl std::fmt::Display for TestbooruPost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lower_case = format!("{:?}" , self).to_lowercase();
        write!(f, "TestbooruPost: id={}", self.id)
    }
}
