use async_trait::async_trait;

use crate::domain::entities::{DiscordUser, RequestData, Session, Token, TokenData};
use crate::domain::exceptions::AuthError;

#[async_trait]
pub trait IOAuthProvider: Send + Sync {
    fn get_authorization_url(&self) -> String;
    fn get_request_data(&self, code: String) -> RequestData;
    async fn exchange_code(&self, code: String) -> Result<TokenData, AuthError>;
    async fn get_user_info(&self, token_data: TokenData) -> Result<DiscordUser, AuthError>;
}

pub trait ITokenService: Send + Sync {
    fn create_access_token(&self, user_id: &str) -> Result<String, AuthError>;
    fn create_refresh_token(&self) -> String;
    fn sign(&self, payload: serde_json::Value) -> Result<String, AuthError>;
}

#[async_trait]
pub trait IStorageRepository: Send + Sync {
    async fn create(
        &self,
        user_id: &str,
        refresh_token: &str,
        ttl: i64,
    ) -> Result<Session, AuthError>;

    async fn get_del(&self, refresh_token: &str) -> Result<Option<Session>, AuthError>;
    async fn delete(&self, user_id: &str, refresh_token: &str) -> Result<(), AuthError>;
    async fn delete_all(&self, user_id: &str) -> Result<u64, AuthError>;
}

#[async_trait]
pub trait IOICService: Send + Sync {
    async fn login(&self, code: &str) -> Result<Token, AuthError>;
    async fn refresh(&self, refresh_token: &str) -> Result<Token, AuthError>;
    async fn logout(&self, user_id: &str, refresh_token: &str) -> Result<(), AuthError>;
}
