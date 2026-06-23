use async_trait::async_trait;

use crate::domain::entities::{DiscordUser, RequestData, Session, Token, TokenData};
use crate::domain::exceptions::AuthError;

#[async_trait]
pub trait IOAuthProvider: Send + Sync {
    fn get_authorization_url(&self) -> String;
    fn get_request_data(&self, code: &str) -> RequestData;
    async fn exchange_code(&self, code: Option<&str>) -> Result<TokenData, AuthError>;
    async fn get_user_info(&self, token_data: &TokenData) -> Result<DiscordUser, AuthError>;
}

pub trait ITokenService: Send + Sync {
    fn create_access_token(&self, user_id: &str) -> String;
    fn create_refresh_token(&self) -> String;
}

#[async_trait]
pub trait IStorageRepository: Send + Sync {
    async fn create(
        &self,
        user_id: &str,
        refresh_token: &str,
        ip_address: &str,
        ttl: i64,
    ) -> Result<Session, AuthError>;

    async fn get(&self, refresh_token: &str) -> Result<Option<Session>, AuthError>;
    async fn delete(&self, user_id: &str, refresh_token: Option<&str>) -> Result<u64, AuthError>;
}

#[async_trait]
pub trait IOICService {
    async fn login(&self, code: &str, ip_address: &str) -> Token;
    async fn refresh(&self, refresh_token: &str, ip_address: &str) -> Token;
    async fn logout(&self, user_id: &str, refresh_token: &str);
}
