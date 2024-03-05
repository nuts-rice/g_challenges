use std::collections::HashMap;


pub const MD5_REDIS_NAME: &str = "md5s";

#[async_trait]
pub trait Md5Db {
    type Record;
    async fn action(&self, record: Self::Record) -> Result<HashMap<String, String>, Error>;
    async fn exists(&self, record: Self::Record) -> Result<Vec<String>, Error>;

}


struct Md5RedisDb {
    pub config: Config,
    pub pool: r2d2::Pool<redis::Client>,
}

struct Md5RedisRecord {
    pub md5: String,
    pub value: String,
}
impl Md5Db for Md5RedisDb {
    type Record = Md5RedisRecord;

    async fn action(&self, record: Self::Record) -> Result<HashMap<String, String>, Error> {
        todo!()
        // let mut conn = self.pool.get().await?;
        // let mut cmd = redis::cmd("HSET");
        // cmd.arg(MD5_REDIS_NAME);
        // cmd.arg(record.md5);
        // cmd.arg(record.value);
        // let _: () = cmd.query_async(&mut *conn).await?;
        // Ok(HashMap::new())
    }
    async fn exists(&self, record: Self::Record) -> Result<Vec<String>, Error> {
        todo!()
        // let mut conn = self.pool.get().await?;
        // let mut cmd = redis::cmd("HGET");
        // cmd.arg(MD5_REDIS_NAME);
        // cmd.arg(record.md5);
        // let value: String = cmd.query_async(&mut *conn).await?;
        // Ok(vec![value])
    }
} 

#[derive(Debug, Clone, Default)]
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
    fn default() -> Self {}
}


impl Md5RedisDb {
    pub fn new(config: Config) -> Self {
        let pool = create_pool(config.clone());
        Self { config, pool }
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
