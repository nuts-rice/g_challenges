use async_trait::async_trait;
use reqwest::Url;
use std::{any::{Any}, collections::HashMap, };

//Any can get type_id for identy

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
enum LoadTagsResult {
    Ok,
    Error,
    CloudflareError,
    NetworkError,
}

pub struct Image {
parent: Page,
parent_url: Url,
width: i32,
height: i32,
id: u64,
file_size: i32,
created_raw: String,
page_url: Url,
file_url: Url,
name: String,
has_tag: bool,
is_valid: bool,


}

impl Image {


fn md5forced() -> String {
    String::from("md5forced") 
}
}
pub struct Page {}
pub struct Profile {}

pub struct Site {}

pub struct ImageBuilder<T: ImageFactory> {
    _marker: std::marker::PhantomData<T>,
}

#[async_trait]
pub trait ImageFactory: From<ImageBuilder<Self>> + Any {
    fn build(&self, site: Site, details: HashMap<String, String>, profile: Profile, parent: Page) -> ImageBuilder<Self>;
    fn build_from_identity(&self, details: HashMap<String, String>, profile: Profile, parent: Page,  
                           // identity: Hashmap??? 
                           ) -> ImageBuilder<Self>;
    fn parse_string(&self) -> Result<String>;
    fn parse_int(&self) -> Result<i32>;
    fn parse_bool(&self) -> Result<bool>;
    fn parse_created_at(&self) ;
    fn parse_date(&self) ;
    fn parse_rating(&self) ;
    fn parse_typed_tags(&self)  ;
    fn parse_tags(&self) ;
}

impl<T: ImageFactory + Any> Default for ImageBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ImageFactory + Any> ImageBuilder<T> {
    pub fn new() -> Self {
        ImageBuilder {
            _marker: std::marker::PhantomData,
        }
    }
    // pub fn create(&self) -> Image {
    //     Image {
    //         parent: Page {},

    //     }
    // }
    fn parse_int(key: String) -> Result<()>{
    Ok(())

    }
    fn parse_string() -> Result<()>  {
Ok(())
    
    }
    fn parse_bool() -> Result<()>{
    Ok(())
    }
    fn parse_typed_tags() -> Result<()> {
    Ok(())

    }  
}

pub fn img_factory() {}
