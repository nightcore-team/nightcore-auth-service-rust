use async_trait::async_trait;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use tracing::debug;

use crate::domain::entities::Session;
use crate::domain::exceptions::AuthError;
use crate::domain::interfaces::IStorageRepository;

pub struct RedisStorageRepository {
    client: ConnectionManager,
}

impl RedisStorageRepository {
    pub fn new(client: ConnectionManager) -> Self {
        Self { client }
    }
    fn session_key(refresh_token: &str) -> String {
        format!("session:{refresh_token}")
    }
    fn user_sessions_key(user_id: &str) -> String {
        format!("user_sessions:{user_id}")
    }
}

#[async_trait]
impl IStorageRepository for RedisStorageRepository {
    async fn create(
        &self,
        user_id: &str,
        refresh_token: &str,
        ttl: i64,
    ) -> Result<Session, AuthError> {
        let session = Session {
            user_id: user_id.to_string(),
        };

        let json = serde_json::to_string(&session)?;
        let mut conn = self.client.clone();

        debug!(user_id = %user_id, ttl = %ttl, "Creating session in Redis");

        let _: () = redis::pipe()
            .set_ex(Self::session_key(refresh_token), &json, ttl as u64)
            .sadd(Self::user_sessions_key(user_id), refresh_token)
            .expire(Self::user_sessions_key(user_id), ttl)
            .query_async(&mut conn)
            .await?;

        Ok(session)
    }

    async fn get_del(&self, refresh_token: &str) -> Result<Option<Session>, AuthError> {
        let mut conn = self.client.clone();
        let data: Option<String> = conn.get_del(Self::session_key(refresh_token)).await?;

        match data {
            Some(json) => {
                let session: Session = serde_json::from_str(&json)?;
                debug!(user_id = %session.user_id, "Session found and deleted from Redis");
                let _: () = conn
                    .srem(Self::user_sessions_key(&session.user_id), refresh_token)
                    .await?;
                Ok(Some(session))
            },
            None => {
                debug!("Session not found in Redis");
                Ok(None)
            },
        }
    }

    async fn delete(&self, user_id: &str, refresh_token: &str) -> Result<(), AuthError> {
        debug!(user_id = %user_id, "Deleting session from Redis");
        let mut conn = self.client.clone();
        let _: () = redis::pipe()
            .del(Self::session_key(refresh_token))
            .srem(Self::user_sessions_key(user_id), refresh_token)
            .query_async(&mut conn)
            .await?;
        Ok(())
    }

    async fn delete_all(&self, user_id: &str) -> Result<u64, AuthError> {
        debug!(user_id = %user_id, "Deleting all user sessions from Redis");
        let mut conn = self.client.clone();
        let tokens: Vec<String> = conn.smembers(Self::user_sessions_key(user_id)).await?;

        if tokens.is_empty() {
            return Ok(0);
        }

        let mut pipe = redis::pipe();
        for token in &tokens {
            pipe.del(Self::session_key(token));
        }
        pipe.del(Self::user_sessions_key(user_id));

        let result: Vec<u64> = pipe.query_async(&mut conn).await?;
        let count: u64 = result.iter().sum();
        Ok(count)
    }
}
