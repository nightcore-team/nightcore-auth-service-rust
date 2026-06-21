use redis::{RedisError, aio::ConnectionManager};

use crate::infra::redis::config::Config as RedisConfig;

pub async fn create_redis_client(config: &RedisConfig) -> Result<ConnectionManager, RedisError> {
    let url = match &config.redis_password {
        Some(pwd) => format!(
            "redis://:{pwd}@{}:{}/{}",
            config.redis_host, config.redis_port, config.redis_db
        ),
        None => format!(
            "redis://{}:{}/{}",
            config.redis_host, config.redis_port, config.redis_db
        ),
    };
    let client = redis::Client::open(url.as_str())?;
    ConnectionManager::new(client).await
}
