// see: https://github.com/Bionus/imgbrd-grabber/blob/master/src/lib/src/models/md5-database
use crate::CrabbooruError;
use async_trait::async_trait;

use fred::{prelude::*, types::Builder};


use serde::{Deserialize, Serialize};
use serde_json::{json};
use std::{
    collections::HashMap,
};

type Result<T> = std::result::Result<T, CrabbooruError>;

#[async_trait]
pub trait Md5Db {
    type Record;
    const NAME: &'static str;
    const COUNT: &'static u32;
    const TIMEOUT: &'static u64;
    const RECONNECT_INTERVAL: &'static u32;
    const POOL_MAX_OPEN: &'static u32;
    async fn action(&self, record: Self::Record) -> Result<HashMap<String, String>>;
    async fn add(&self, record: Self::Record) -> Result<()>;
    async fn connect(&self) -> Result<RedisClient>;
    async fn get_conn(&self) -> Result<RedisClient>;
    async fn exists(&self, record: Self::Record) -> Result<Vec<String>>;
    async fn sync(&self) -> Result<()>;
    async fn remove(&self, record: Self::Record) -> Result<()>;
    async fn count(&self) -> Result<u64>;
    async fn paths(&self) -> Result<Vec<String>>;
}

struct Md5RedisDb {
    pub config: RedisConfig,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Md5RedisRecord {
    pub md5: String,
    pub path: String,
}

#[async_trait]
impl Md5Db for Md5RedisDb {
    type Record = Md5RedisRecord;
    const NAME: &'static str = "md5_redis";
    const TIMEOUT: &'static u64 = &1;
    const RECONNECT_INTERVAL: &'static u32 = &100;
    const POOL_MAX_OPEN: &'static u32 = &10;
    const COUNT: &'static u32 = &0;
    async fn action(&self, _record: Self::Record) -> Result<HashMap<String, String>> {
        todo!()
        // let mut conn = self.pool.get().await?;
        // let mut cmd = redis::cmd("HSET");
        // cmd.arg(MD5_REDIS_NAME);
        // cmd.arg(record.md5);
        // cmd.arg(record.value);
        // let _: () = cmd.query_async(&mut *conn).await?;
        // Ok(HashMap::new())
    }

    async fn get_conn(&self) -> Result<RedisClient> {
        let client = Builder::from_config(self.config.clone()).build().unwrap();
        client.init().await.expect("Error initializing client");
        Ok(client)
    }
    //TODO: type error with .set
    async fn add(&self, _record: Self::Record) -> Result<()> {
        let _conn = self.get_conn().await?;
        if _record.md5.is_empty() {
            return Ok(());
        }
        let _record = json!(_record);

        // let _: () = conn.set("md5", record.to_string(), None, None, false).await.expect("Error setting md5")?;
        // conn.set("md5", record.to_string(), None, None, false).await.expect("Error setting md5")?;
        // let _: () = redis::cmd("SET").arg(_record.channel).arg(_record.md5).query_async(conn.deref_mut()).unwrap();
        //.set(_record.channel, _record.md5);

        // let _: () = redisuse redis_macros::{FromRedisValue, ToRedisArgs};::cmd("SET").arg(_record.channel).arg(_record.md5);

        Ok(())
        // let mut conn = self.pool.get().await?;
        // let mut cmd = redis::cmd("HSET");
        // cmd.arg(MD5_REDIS_NAME);
        // cmd.arg(record.md5);
        // cmd.arg(record.value);
        // let _: () = cmd.query_async(&mut *conn).await?;
        // Ok(())
    }
    async fn connect(&self) -> Result<RedisClient> {
        unimplemented!()
        // let manager =
        //     RedisConnectionManager::new(self.config.host.as_str()).expect("Redis client error");
        // let pool = r2d2::Pool::builder()
        //     .max_size(*Self::POOL_MAX_OPEN)
        //     // .max_lifetime(Some((*Self::TIMEOUT as i64)))
        //     .build(manager)
        // .map_err(|e| r2d2::Error::from(Some(e.to_string())))
        // .unwrap();

        // Ok(pool)
    }
    async fn exists(&self, _record: Self::Record) -> Result<Vec<String>> {
        todo!()
        // let mut conn = self.pool.get().await?;
        // let mut cmd = redis::cmd("HGET");
        // cmd.arg(MD5_REDIS_NAME);
        // cmd.arg(record.md5);
        // let value: String = cmd.query_async(&mut *conn).await?;
        // Ok(vec![value])
    }
    async fn sync(&self) -> Result<()> {
        todo!()
    }
    async fn remove(&self, _record: Self::Record) -> Result<()> {
        todo!()
    }
    async fn count(&self) -> Result<u64> {
        todo!()
    }
    async fn paths(&self) -> Result<Vec<String>> {
        todo!()
    }
}

impl Drop for Md5RedisDb {
    fn drop(&mut self) {
        todo!()
    }
}

impl Md5RedisDb {
    // pub fn new(_config: Config) -> Self {
    //     todo!()
    //     // let pool =
    //     // Self { config, pool }
    // }
    async fn get_query(&self, _key: &str) -> Result<Vec<Md5RedisRecord>> {
        todo!()
    }
    async fn add_query(&self, _key: &str, _value: &str) -> Result<()> {
        todo!()
    }
    async fn remove_query(&self, _key: &str) -> Result<()> {
        todo!()
    }
    async fn count_query(&self, _key: &str) -> Result<u64> {
        todo!()
    }
    async fn remove_all_query(&self) -> Result<()> {
        todo!()
    }
}

// #[derive(Debug, Clone)]
// struct Config {
//     pub host: String,
//     pub port: u16,
//     pub password: String,
//     pub db: u8,
// }

// impl Default for Config {
//     fn default() -> Self {
//         Self {
//             host: "localhost:6379".to_string(),
//             port: 6379,
//             password: "passwrd".to_string(),
//             db: 0,
//         }
//     }
// }

impl Md5RedisDb {
    pub fn new(_config: RedisConfig) -> Self {
        todo!()
        // let pool =
        // Self { config, pool }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_md5_redis_db() {
        let config = RedisConfig::default();
        let md5_redis_db = Md5RedisDb::new(config);
    }
}
