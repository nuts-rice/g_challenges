use reqwest::prelude::*;
use crate::api::*;
use crate::model::*;

fn setup_md5db() -> Md5Db {
    todo!()
}

fn setup_api() -> Result<Client> {
    let res = reqwest::Client::new();

}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    fn test_md5db() {
        let md5db = setup_md5db();
        assert_eq!(md5db.settings.file, "md5db.txt");
    }

    #[tokio::test]
    fn test_booru_endpoint_get() {
        let url = "testbooru.donmai.us/posts.json";
        let tags = ["touhou", "reiuji_utsuho"].join(" ");
        let query = api::testbooru_call(tags, 1, 20);
        let request = query.unwrap();
        assert!(request.status().is_success());
    }
}
