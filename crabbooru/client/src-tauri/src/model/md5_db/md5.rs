use clap::{Parser};

pub struct Md5Db {
    pub settings: Md5DbSettings,

}


#[derive(Parser, Debug)]
pub struct Md5DbSettings {
    pub file: String,
    pub output: String,
    pub verbose: bool,
    pub threads: u8,
}


impl Md5Db {
    pub fn new(settings: Md5DbSettings) -> Self {
        Md5Db {
            settings
        }
    }
}


