use tauri::async_runtime::RwLock;
use std::sync::Arc;
use async_trait::async_trait;
use reqwest::{header, header::{HeaderMap,USER_AGENT}, Error, };
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::HashMap};
use tauri::State;
use tracing::{debug, info};
use url::Url;
use crate::error::CrabbooruError;
use crate::model::{DanbooruPost, TestbooruPost, SafebooruPost};
const TEST_URL: &'static str = "https://testbooru.donmai.us";
const DANBOORU_URL: &'static str = "https://danbooru.donmai.us";
pub const USR_USER_AGENT: &str = ("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3");

type Result<T> = std::result::Result<T, CrabbooruError>;
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
#[async_trait]
pub trait ApiClient {
    type Post;
    const URL: &'static str;
    const SORT: &'static str;
    async fn booru_call(&self, tags: Vec<String>, limit: u32, page: u32) -> Result<Vec<Self::Post>>;
    async fn booru_call_id(&self, id: u32) -> Result<Self::Post>;

}

    // ctx: PooledContext,
    // pub testbooru: TestbooruClient,
    // pub danbooru: DanbooruClient,

// impl Clone for ApiClient {
//     fn clone(&self) -> Self {
//         Self {
//             ctx: self.ctx.clone(),
//             testbooru: self.testbooru.clone(),
//             danbooru: self.danbooru.clone(),
//         }
//     }
// }
// impl ApiClient {
// pub fn new(ctx: PooledContext) -> Self {
//     Self {
//         ctx,
//         testbooru: TestbooruClient::from(ApiBuilder::<TestbooruClient>::new()),
//         danbooru: DanbooruClient::from(ApiBuilder::<DanbooruClient>::new()),
//     }
// }


// }
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

// pub type Image = Post;
// pub type Page = ParsedPage;
// pub type Site = PageUrl;

pub type DanbooruAccess<'a> = State<'a, DanbooruClient>;
pub type TestbooruAccess<'a> = State<'a, TestbooruClient>;
#[async_trait]
pub trait Api: From<ApiBuilder<Self>> + Any {
    //TODO: generalize this
    type Image;
    // type Page;
    // type Site;
    const URL: &'static str;
    const SORT: &'static str;

    fn builder() -> ApiBuilder<Self> {
        ApiBuilder::new()
    }
    async fn get(&self) -> Result<Vec<Self::Image>>;
    async fn get_by_id(&self, id: u32) -> Result<Self::Image>;
    async fn api(&self) -> Result<Self>;
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
            Err(e) => Err(CrabbooruError{message: "failed to get page".to_string()}),}
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
            .await
            .or(Err(format!("Failed to GET from '{}'", &url)))
            .unwrap()
            .json::<Vec<Post>>()
            .await;
        //JSON::Vec<I::Image>
        Ok(response.unwrap())
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

    pub async fn api(&self) -> Self {
        todo!()
        // self.build().api().await
    }

    pub fn set_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    //IMAGES
    async fn parse_image<P: Api + Any, I: Api + Any>(
        &self,
        _site: PageUrl,
        // _parentPage: P::Page,
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
    ) -> Result<()> {
        todo!()
            // let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
            // let client = self.client.clone();

            // let id = 5942;
            // let response = client.get(format!("https://testbooru.donmai.us/posts/{}", &id)).header(USER_AGENT, user_agent).send().await.or(Err(format!("Failed to GET from '{}'", &id))).unwrap();
            // Ok(image)





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
    // fn get_site_headers<S: Api + Any>(&self, _site: S::Site) -> HashMap<String, String> {
    //     HashMap::new()
    // }
}
pub fn get_headers() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        header::HeaderValue::from_static("PostmanRuntime/7.30.0"),
    );
    headers
}
pub struct DanbooruClient {
    pub inner: ApiBuilder<Self> 
}

#[tauri::command]
pub async fn danbooru_call(tags: Vec<String>, page: u32, limit: u32, ) -> Result<Vec<DanbooruPost>> {
    let url = format!("{DANBOORU_URL}/posts.json");
    let _tags = tags.join(" ");
    info!("Danbooru Tags: {}", &_tags);
    info!("Danbooru URL: {}", &url);
    let query = reqwest::Client::new().get(url).headers(get_headers()).query(&[("limit", limit.to_string().as_str()),("page", page.to_string().as_str()),( "tags", &_tags)]);
info!("Danbooru Query: {:?}", query);
let response = query.send().await.unwrap().json::<Vec<DanbooruPost>>().await.unwrap();
    

    Ok(response)
}

impl From<ApiBuilder<Self>> for DanbooruClient {
    fn from(builder: ApiBuilder<Self>) -> Self {
        Self{inner: builder}
    }
}
unsafe impl Send for DanbooruClient {}
unsafe impl Sync for DanbooruClient {}

#[async_trait]
impl Api for DanbooruClient {
    type Image = DanbooruPost;
    // type Page = Page;
    // type Site = Site;
    const URL: &'static str = "https://danbooru.donmai.us";
    const SORT: &'static str = "date:";
    async fn api(&self) -> Result<DanbooruClient> {
        todo!()
        // let inner = self.inner.read().await;
        // inner.clone()
        // .ok_or_else(|| CrabbooruError::from("not connected"))
    }

    async fn get(&self) -> Result<Vec<Self::Image>> {
        todo!()
        // let builder = &self.inner;
        // let tag_string = builder.tags.join(" ");
        // let url = builder.url.as_str();
        // let response = builder
        //     .client
        //     .get(format!("{url}/posts.json"))
        //     .headers(get_headers())
        //     .query(&[
        //         ("limit", builder.limit.to_string().as_str()),
        //         ("tags", &tag_string),
        //     ])
        //     .send()
        //     .await?
        //     .json::<Vec<Image>>()
        //     .await?;

        // Ok(response)
    }
    async fn get_by_id(&self, id: u32) -> Result<Self::Image> {
        let builder = &self.inner;
        let url = builder.url.as_str();
        let response = builder
            .client
            .get(format!("{url}/posts/{id}.json"))
            .headers(get_headers())
            .send()
            .await
            .or(Err(format!("Failed to GET from '{}'", &id)))
            .unwrap()
            .json::<DanbooruPost>()
            .await;
        Ok(response.unwrap())

    }
}

pub struct TestbooruClient {
    pub inner: ApiBuilder<Self> 
}
#[tauri::command]
pub async fn testbooru_call(tags: Vec<String>, page: u32, limit: u32, ) -> 
//Result<Vec<TestbooruPost>> 
Result<Vec<TestbooruPost>>{
    let url = format!("{TEST_URL}/posts.json");
    let _tags = tags.join(" ");
    info!("Test Tags: {}", &_tags);
    info!("Test URL: {}", &url);
    let query = reqwest::Client::new()
    .get(url)
    .headers(get_headers())
    .query(&[
        ("limit", limit.to_string().as_str()),
        ("page", page.to_string().as_str()),
        ("tags", &_tags),
        ]);
    info!("Test Query: {:?}", query);
let response = query.send().await.unwrap().json::<Vec<TestbooruPost>>().await.unwrap();
info!("Test Response: {:?}", response);
    Ok(response)
}
#[tauri::command]
pub async fn testbooru_call_id(id:u32) -> Result<TestbooruPost> {
    let url = format!("{TEST_URL}/posts/{id}.json");
    let res = reqwest::Client::new().get(url).headers(get_headers()).send().await.unwrap().json::<TestbooruPost>().await.unwrap();
    info! ("testbooru_call_id query: {:?}", res);
    Ok(res)
}

    // let response = reqwest::Client::new().get(url).query(&["limit", limit, "tags", query]).send().await.unwrap();
    
    // match response.status() {
    //     reqwest::StatusCode::OK => {
    //             let parsed = response.json::<Vec<TestbooruPost>>().await.unwrap();
                
    //         return Ok(parsed)
    //     },
    //     _ =>{
    //     let parsed: CrabbooruError = response.json().await.unwrap();
    //     return Err(parsed)
    //     }

    // }


impl From<ApiBuilder<Self>> for TestbooruClient {
    fn from(builder: ApiBuilder<Self>) -> Self {
        Self{inner: builder}
    }
}


    // pub inner: Arc<RwLock<Option<ApiBuilder<Self>>>>}

// unsafe impl Send for TestbooruClient {}
// unsafe impl Sync for TestbooruClient {}
#[async_trait]
impl Api for TestbooruClient {
    type Image = TestbooruPost;
    // type Page = Page;
    // type Site = Site;
    const URL: &'static str = "https://testbooru.donmai.us";
    const SORT: &'static str = "date:";
    async fn api(&self) -> Result<TestbooruClient> {
        todo!()
        // let inner = self.inner.read().await;
        // inner.clone()
        // .ok_or_else(|| CrabbooruError::from("not connected"))
    }
    async fn get(&self) -> Result<Vec<Self::Image>> {
        let builder = &self.inner;
        let tag_string = builder.url.as_str();
        let url = builder.url.as_str();
        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
        let response = builder.client.get(format!("{url}/posts.json")).header(USER_AGENT, user_agent).query(&[
                                                                                                            "limit", builder.limit.to_string().as_str(),
                                                                                                            "tags", &tag_string,
                                                                                                                                                                                                                    
        ]).send().await.or(Err(format!("Failed to GET from '{}'", &url))).unwrap().json::<Vec<TestbooruPost>>().await;
        Ok(response.unwrap())


        // let builder = &self.inner;
        // let tag_string = builder.tags.join(" ");
        // let url = builder.url.as_str();
        // let response = builder
        //     .client
        //     .get(format!("{url}/posts.json"))
        //     .headers(get_headers())
        //     .query(&[
        //         ("limit", builder.limit.to_string().as_str()),
        //         ("tags", &tag_string),
        //     ])
        //     .send()
        //     .await?
        //     .json::<Vec<Image>>()
        //     .await?;

        // Ok(response)
    }
   async fn get_by_id(&self, id: u32) -> Result<Self::Image> {
    let builder = &self.inner;
    let url = builder.url.as_str();
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
    let response = builder.client.get(format!("{url}/posts/{id}.json")).header(USER_AGENT, user_agent).send().await.or(Err(format!("Failed to GET from '{}'", &id))).unwrap().json::<TestbooruPost>().await;
    Ok(response.unwrap())

    
        // let builder = &self.inner;
        // let tag_string = builder.tags.join(" ");
        // let url = builder.url.as_str();
        // let response = builder
        //     .client
        //     .get(format!("{url}/posts/{id}.json"))
        //     .headers(get_headers())
        //     .query(&[
        //         ("limit", builder.limit.to_string().as_str()),
        //         ("tags", &tag_string),
        //     ])
        //     .send()
        //     .await?
        //     .json::<Vec<Image>>()
        //     .await?;

        // Ok(response)
    }
}

pub struct SafebooruClient {
    pub inner: reqwest::Client,
}
#[async_trait]
impl ApiClient for SafebooruClient {
    type Post = SafebooruPost;
    const URL: &'static str = "https://safebooru.donmai.us";
    const SORT: &'static str = "sort:";

    async fn booru_call_id(&self, id: u32, ) -> Result<SafebooruPost> {
                todo!()

    }


    async fn booru_call(&self, tags: Vec<String>, page: u32, limit: u32, ) -> Result<Vec<SafebooruPost>> {
        let _url = Self::URL;
        let url = format!("{_url}/index.php");
        let _tags = tags.join(" ");
        info!("SafeBooru Tags: {}", &_tags);
        info!("SafeBooru URL: {}", &url);
        let query = self.inner
        .get(url)
        .headers(get_headers())
        .query(&[
            ("page", "dapi"),
            ("s", "post"),
            ("q", "index"),
            ("pid", page.to_string().as_str()),
            ("limit", limit.to_string().as_str()),
            ("tags", &_tags),
            ("json", "1"),
            ]);
        info!("Safe Query: {:?}", query);
        let response = query.send().await.unwrap().json::<Vec<SafebooruPost>>().await.unwrap();
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
