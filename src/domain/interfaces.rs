use std::collections::HashMap;

use async_trait::async_trait;

use crate::domain::entities::{Session, Token, TokenData};
use crate::domain::exceptions::AuthError;

#[async_trait]
pub trait IOAuthProvider {
    fn get_authorization_url(&self) {}
    async fn exchange_core(&self, code: Option<&String>) {}
    async fn get_user_info(&self, token_data: &TokenData) {}
}

pub trait ITokenService {
    fn create_access_token(&self, user_id: &str) {}
    fn create_refresh_token(&self) {}
    fn sigh(&self, payload: HashMap<String, i64>) {}
}

#[async_trait]
pub trait IStorageRepository {
    async fn create(
        &self,
        user_id: &str,
        refresh_token: &str,
        ip_address: &str,
        ttl: i64,
    ) -> Result<Session, AuthError>;

    async fn get(&self, refresh_token: &str) -> Result<Option<String>, AuthError>;
    async fn delete(&self, user_id: &str, refresh_token: Option<&str>) -> Result<u64, AuthError>;
}

#[async_trait]
pub trait IOICService {
    fn oauth_provider(&self) -> &dyn IOAuthProvider;
    fn token_service(&self) -> &dyn ITokenService;
    fn storage(&self) -> &dyn IStorageRepository;

    async fn login(&self, code: &str, ip_address: &str) -> Token;
    async fn refresh(&self, refresh_token: &str, ip_address: &str) -> Token;
    async fn logout(&self, refresh_token: &str);
}
