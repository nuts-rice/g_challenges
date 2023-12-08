use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct PageUrl {
    error: String,
    url: String,
    headers: HashMap<String, String>,
}

pub struct ParsedPage {
    error: String,
    pagecount: u32,
    imgcount: u32,
    tags: Vec<String>,
    // images: Vec<Image>,
    nextpage: String,
    prevpage: String,
    url: String,
    headers: HashMap<String, String>,
}

pub struct ApiBuilder<T: Api> {
    client: reqwest::Client,
    key: Option<String>,
    user: Option<String>,
    tags: Vec<String>,
    limit: u32,
    _marker: std::marker::PhantomData<T>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: u32,
    pub tags: String,
    pub created_at: String,
    pub creator_id: u32,
    pub image_width: u32,
    pub image_height: u32,
    pub score: i32,
    pub file_ext: String,
    pub file_size: u32,
}

#[async_trait]
pub trait Api: From<ApiBuilder<Self>> + Any {
    type Image;
    type Page;
    type Site;
    fn builder() -> ApiBuilder<Self> {
        ApiBuilder::new()
    }
    async fn get_name(&self) -> String;
    async fn set_credientials(&mut self, key: String, user: String) -> Self;
    async fn get_page(&self, url: String, headers: HashMap<String, String>) -> PageUrl;
    async fn parse_page(&self, page: PageUrl) -> ParsedPage;
    async fn parse_image(
        &self,
        site: Self::Site,
        parentPage: Self::Page,
        position: u32,
        tags: Vec<String>,
    ) -> Option<Self::Image>;
    async fn get_image(&self, url: String, headers: HashMap<String, String>) -> Self::Image;
    async fn get_site(&self, url: String, headers: HashMap<String, String>) -> Self::Site;
    async fn get_image_url(&self, image: Self::Image) -> String;
    async fn get_image_data(&self, image: Self::Image) -> Vec<u8>;
    async fn get_image_headers(&self, image: Self::Image) -> HashMap<String, String>;
}
impl<T: Api + Any> Default for ApiBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Api + Any> ApiBuilder<T> {
    pub fn new() -> Self {
        ApiBuilder {
            client: reqwest::Client::new(),
            key: None,
            user: None,
            tags: Vec::new(),
            limit: 100,
            _marker: std::marker::PhantomData,
        }
    }
    fn get_name(&self) -> String {
        String::from("")
    }
    pub fn set_credientials(mut self, key: String, user: String) -> Self {
        self.key = Some(key);
        self.user = Some(user);
        self
    }
    pub fn get_page(&self, _url: String, _headers: HashMap<String, String>) -> PageUrl {
        PageUrl {
            error: String::from(""),
            url: String::from(""),
            headers: HashMap::new(),
        }
    }

    async fn get_image<I: Api + Any, S: Api + Any>(
        &self,
        page: PageUrl,
    ) -> Result<Vec<Post>, reqwest::Error> {
        let url = page.url.as_str();
        let tags = self.tags.join(" ");
        let response = self
            .client
            .get(format!("{url}/posts.json"))
            .query(&[
                ("limit", self.limit.to_string().as_str()),
                ("tags", tags.as_str()),
            ])
            .send()
            .await?
            .json::<Vec<Post>>()
            .await?;
        //JSON::Vec<I::Image>
        Ok(response)
    }

    pub fn parse_page(&self, _page: PageUrl) -> ParsedPage {
        ParsedPage {
            error: String::from(""),
            pagecount: 0,
            imgcount: 0,
            tags: Vec::new(),
            nextpage: String::from(""),
            prevpage: String::from(""),
            url: String::from(""),
            headers: HashMap::new(),
        }
    }
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        //Check for danbooru
        // if TypeId::of::<T>
        self.tags.push(tag.into());
        self
    }

    pub fn build(self) -> T {
        T::from(self)
    }

    //IMAGES
    async fn parse_image<P: Api + Any, I: Api + Any>(
        &self,
        _site: PageUrl,
        _parentPage: P::Page,
        _position: u32,
        _tags: Vec<String>,
    ) -> Option<I::Image> {
        // Option<I::Image> {
        unimplemented!()
    }
    async fn get_image_data<I: Api + Any>(&self, _image: I::Image) -> Vec<u8> {
        Vec::new()
    }
    async fn get_image_test<I: Api + Any>(
        &self,
        _url: String,
        _headers: HashMap<String, String>,
    ) -> Option<I::Image> {
        unimplemented!()
    }
    // fn get_site(&self, _url: String, _headers: HashMap<String, String>) -> Site {
    //     Site {}
    // }
    // fn get_image_url(&self, _image: Image) -> String {
    //     String::from("")
    // }
    // fn get_image_data(&self, _image: Image) -> Vec<u8> {
    //     Vec::new()
    // }
    // fn get_image_headers(&self, _image: Image) -> HashMap<String, String> {
    //     HashMap::new()
    // }
    // fn get_site_url(&self, _site: Site) -> String {
    //     String::from("")
    // }
    // fn get_site_data(&self, _site: Site) -> Vec<u8> {
    //     Vec::new()
    // }
    fn get_site_headers<S: Api + Any>(&self, _site: S::Site) -> HashMap<String, String> {
        HashMap::new()
    }
}
//Danbooru
// pub struct DanbooruClient(ApiBuilder<Self>)

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn posts_with_tag_test() {
        unimplemented!()
    }
}
