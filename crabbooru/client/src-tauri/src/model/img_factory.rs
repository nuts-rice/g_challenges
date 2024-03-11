use async_trait::async_trait;
use image::RgbImage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Img {
    md5: String,
    tags: Vec<String>,
    file_size: u64,
}

#[async_trait]
pub trait Image {}



pub struct ImgFactory {
}
