use async_trait::async_trait;
use reqwest::{header, header::HeaderMap, Error};
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::HashMap};
type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Deserialize, Serialize, Clone, )]
pub struct PageUrl {
    pub error: String,
    pub url: String,
    pub headers: HashMap<String, String>,
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
    url: String,
    _marker: std::marker::PhantomData<T>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: u32,
    pub tags: Vec<String>,
    pub created_at: String,
    pub creator_id: u32,
    pub image_width: u32,
    pub image_height: u32,
    pub score: i32,
    pub file_ext: String,
    pub file_size: u32,
}

pub type Image = Post;
pub type Page = ParsedPage;
pub type Site = PageUrl;

#[async_trait]
pub trait Api: From<ApiBuilder<Self>> + Any {
    //TODO: generalize this
    type Image;
    type Page;
    type Site;
    const URL: &'static str;
    const SORT: &'static str;

    fn builder() -> ApiBuilder<Self> {
        ApiBuilder::new()
    }
    async fn get(&self) -> Result<Vec<Self::Image>>;
    async fn get_by_id(&self, id: u32) -> Result<Vec<Self::Image>>;
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
            url: T::URL.to_string(),
            _marker: std::marker::PhantomData,
        }
    }
    fn get_name(&self) -> String {
        String::from("")
    }

    pub fn get_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("PostmanRuntime/7.30.0"),
        );
        headers
    }

    pub fn default_url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }
    pub fn set_credientials(mut self, key: String, user: String) -> Self {
        self.key = Some(key);
        self.user = Some(user);
        self
    }
    pub async fn get_page(&self, _url: String, _headers: HashMap<String, String>) -> Result<PageUrl> {
        let result = self.client.get(_url.as_str()).send().await;
        match result {
            Ok(result) => {
                if result.status().is_success() {
                    Ok(PageUrl {
                        error: String::new(),
                        url: _url,
                        headers: _headers,
                    })
                } else {
                    todo!()
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_image(&self, page: PageUrl) -> Result<Vec<Post>> {
        let url = page.url.as_str();
        let tags = self.tags.join(" ");
        let response = self
            .client
            .get(format!("{url}/posts.json"))
            .headers(get_headers())
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

    pub fn set_url(mut self, url: String) -> Self {
        self.url = url;
        self
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
pub fn get_headers() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("PostmanRuntime/7.30.0"),
    );
    headers
}
pub struct DanbooruClient(ApiBuilder<Self>);
impl From<ApiBuilder<Self>> for DanbooruClient {
    fn from(builder: ApiBuilder<Self>) -> Self {
        Self(builder)
    }
}
#[async_trait]
impl Api for DanbooruClient {
    type Image = Image;
    type Page = Page;
    type Site = Site;
    const URL: &'static str = "https://danbooru.donmai.us";
    const SORT: &'static str = "date:";
    async fn get(&self) -> Result<Vec<Image>> {
        let builder = &self.0;
        let tag_string = builder.tags.join(" ");
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/posts.json"))
            .headers(get_headers())
            .query(&[
                ("limit", builder.limit.to_string().as_str()),
                ("tags", &tag_string),
            ])
            .send()
            .await?
            .json::<Vec<Image>>()
            .await?;

        Ok(response)
    }
    async fn get_by_id(&self, _id: u32) -> Result<Vec<Image>> {
        unimplemented!()
    }
}
pub struct TestbooruClient(ApiBuilder<Self>);
impl From<ApiBuilder<Self>> for TestbooruClient {
    fn from(builder: ApiBuilder<Self>) -> Self {
        Self(builder)
    }
}
#[async_trait]
impl Api for TestbooruClient {
    type Image = Image;
    type Page = Page;
    type Site = Site;
    const URL: &'static str = "https://testbooru.donmai.us";
    const SORT: &'static str = "date:";
    async fn get(&self) -> Result<Vec<Image>> {
        let builder = &self.0;
        let tag_string = builder.tags.join(" ");
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/posts.json"))
            .headers(get_headers())
            .query(&[
                ("limit", builder.limit.to_string().as_str()),
                ("tags", &tag_string),
            ])
            .send()
            .await?
            .json::<Vec<Image>>()
            .await?;

        Ok(response)
    }
   async fn get_by_id(&self, id: u32) -> Result<Vec<Image>> {
        let builder = &self.0;
        let tag_string = builder.tags.join(" ");
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/posts/{id}.json"))
            .headers(get_headers())
            .query(&[
                ("limit", builder.limit.to_string().as_str()),
                ("tags", &tag_string),
            ])
            .send()
            .await?
            .json::<Vec<Image>>()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn posts_with_tag_test() {
        let tags = vec!["muscular_female".to_string(), "dorohedoro".to_string()];
        let builder = ApiBuilder::<TestbooruClient>::new()
            .tag(tags[0].clone())
            .tag(tags[1].clone());
        let _api = builder.build().get().await;
    }
}
