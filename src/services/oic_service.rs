use std::sync::Arc;

use async_trait::async_trait;

use crate::core::config::AppConfig;
use crate::domain::entities::Token;
use crate::domain::exceptions::AuthError;
use crate::domain::interfaces::{IOAuthProvider, IOICService, IStorageRepository, ITokenService};

pub struct OICService {
    pub oauth_provider: Arc<dyn IOAuthProvider>,
    pub token_service: Arc<dyn ITokenService>,
    pub storage: Arc<dyn IStorageRepository>,
    pub config: Arc<AppConfig>,
}

impl OICService {
    pub fn new(
        oauth_provider: Arc<dyn IOAuthProvider>,
        token_service: Arc<dyn ITokenService>,
        storage: Arc<dyn IStorageRepository>,
        config: Arc<AppConfig>,
    ) -> Self {
        Self {
            oauth_provider,
            token_service,
            storage,
            config,
        }
    }
}

#[async_trait]
impl IOICService for OICService {
    async fn login(&self, code: &str) -> Result<Token, AuthError> {
        let token_data = self
            .oauth_provider
            .exchange_code(String::from(code))
            .await?;
        let user_info = self.oauth_provider.get_user_info(token_data).await?;

        let access_token = self.token_service.create_access_token(&user_info.id)?;
        let refresh_token = self.token_service.create_refresh_token();
        let refresh_token_max_age = self.config.jwt.jwt_refresh_token_expire_days * 24 * 3600;

        self.storage.delete_all(&user_info.id).await?;
        self.storage
            .create(&user_info.id, &refresh_token, refresh_token_max_age)
            .await?;

        Ok(Token {
            access_token: access_token,
            refresh_token: refresh_token,
            refresh_token_max_age: refresh_token_max_age,
        })
    }

    async fn refresh(&self, refresh_token: &str) -> Result<Token, AuthError> {
        let session = self.storage.get_del(refresh_token).await?;

        let Some(session) = session else {
            return Err(AuthError::SessionNotFound("Session not found".into()));
        };

        let access_token = self.token_service.create_access_token(&session.user_id)?;
        let refresh_token = self.token_service.create_refresh_token();
        let refresh_token_max_age = self.config.jwt.jwt_refresh_token_expire_days * 24 * 3600;

        self.storage
            .create(&session.user_id, &refresh_token, refresh_token_max_age)
            .await?;

        Ok(Token {
            access_token: access_token,
            refresh_token: refresh_token,
            refresh_token_max_age: refresh_token_max_age,
        })
    }

    async fn logout(&self, user_id: &str, refresh_token: &str) -> Result<(), AuthError> {
        self.storage.delete(user_id, refresh_token).await?;
        Ok(())
    }
}
