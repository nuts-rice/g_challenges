use crate::CrabbooruError;
use async_trait::async_trait;

use std::collections::HashMap;


#[async_trait]
pub trait Md5Db {
    type Record;
    const NAME: &'static str;
    const COUNT: &'static u64;
    async fn action(&self, record: Self::Record)
        -> Result<HashMap<String, String>, CrabbooruError>;
    async fn exists(&self, record: Self::Record) -> Result<Vec<String>, CrabbooruError>;
    async fn sync(&self) -> Result<(), CrabbooruError>;
    async fn remove(&self, record: Self::Record) -> Result<(), CrabbooruError>;
    async fn count(&self) -> Result<u64, CrabbooruError>;
    async fn paths(&self) -> Result<Vec<String>, CrabbooruError>;
}

struct Md5RedisDb {
    pub config: Config,
    pub pool: r2d2::Pool<redis::Client>,
}

struct Md5RedisRecord {
    pub md5: String,
    pub value: String,
}

#[async_trait]
impl Md5Db for Md5RedisDb {
    type Record = Md5RedisRecord;
    const NAME: &'static str = "md5_redis";
    const COUNT: &'static u64 = &0;
    async fn action(
        &self,
        _record: Self::Record,
    ) -> Result<HashMap<String, String>, CrabbooruError> {
        todo!()
        // let mut conn = self.pool.get().await?;
        // let mut cmd = redis::cmd("HSET");
        // cmd.arg(MD5_REDIS_NAME);
        // cmd.arg(record.md5);
        // cmd.arg(record.value);
        // let _: () = cmd.query_async(&mut *conn).await?;
        // Ok(HashMap::new())
    }
    async fn exists(&self, _record: Self::Record) -> Result<Vec<String>, CrabbooruError> {
        todo!()
        // let mut conn = self.pool.get().await?;
        // let mut cmd = redis::cmd("HGET");
        // cmd.arg(MD5_REDIS_NAME);
        // cmd.arg(record.md5);
        // let value: String = cmd.query_async(&mut *conn).await?;
        // Ok(vec![value])
    }
    async fn sync(&self) -> Result<(), CrabbooruError> {
        todo!()
    }
    async fn remove(&self, _record: Self::Record) -> Result<(), CrabbooruError> {
        todo!()
    }
    async fn count(&self) -> Result<u64, CrabbooruError> {
        todo!()
    }
    async fn paths(&self) -> Result<Vec<String>, CrabbooruError> {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Config {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub db: u8,
    pub pool_size: u16,
    pub timeout: u16,
    pub max_reconnects: u8,
    pub reconnect_interval: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost:6379".to_string(),
            port: 6379,
            password: "passwrd".to_string(),
            db: 0,
            pool_size: 10,
            timeout: 10,
            max_reconnects: 3,
            reconnect_interval: 5,
        }
    }
}

impl Md5RedisDb {
    pub fn new(_config: Config) -> Self {
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
        let config = Config::default();
        let md5_redis_db = Md5RedisDb::new(config);
    }
}
