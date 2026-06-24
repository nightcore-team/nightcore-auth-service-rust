use std::sync::Arc;

use async_trait::async_trait;

use crate::core::config::AppConfig;
use crate::domain::entities::Token;
use crate::domain::interfaces::{
    IOAuthProvider, IOICService, /* IStorageRepository, */ ITokenService,
};

pub struct OICService {
    pub oauth_provider: Arc<dyn IOAuthProvider>,
    pub token_service: Arc<dyn ITokenService>,
    // pub storage: Arc<dyn IStorageRepository>,
    pub config: Arc<AppConfig>,
}

impl OICService {
    pub fn new(
        oauth_provider: Arc<dyn IOAuthProvider>,
        token_service: Arc<dyn ITokenService>,
        // storage: Arc<dyn IStorageRepository>,
        config: Arc<AppConfig>,
    ) -> Self {
        Self {
            oauth_provider,
            token_service,
            // storage,
            config,
        }
    }
}

#[async_trait]
impl IOICService for OICService {
    async fn login(&self, code: &str, ip_address: &str) -> Token {
        let token_data = self.oauth_provider.exchange_code(String::from(code)).await;
        let user_info = self
            .oauth_provider
            .get_user_info(token_data.unwrap())
            .await
            .unwrap();

        let access_token = self.token_service.create_access_token(&user_info.id);
        let refresh_token = self.token_service.create_refresh_token();
        let refresh_token_max_age = self.config.jwt.jwt_refresh_token_expire_days * 24 * 3600;

        // self.storage.delete(&user_info.id, None).await.unwrap();
        // self.storage
        //     .create(&user_info.id, &refresh_token, ip_address, refresh_token_max_age)
        //     .await
        //     .unwrap();

        Token {
            access_token: access_token,
            refresh_token: refresh_token,
            refresh_token_max_age: refresh_token_max_age,
        }
    }

    async fn refresh(&self, refresh_token: &str, ip_address: &str) -> Token {
        // let session = self.storage.get(refresh_token).await.unwrap().unwrap();

        // if session.ip_address != ip_address {
        //     self.storage
        //         .delete(&session.user_id, Some(refresh_token))
        //         .await
        //         .unwrap();
        //     panic!("Invalid or revoked token")
        // }

        // let keys_count = self
        //     .storage
        //     .delete(&session.user_id, Some(refresh_token))
        //     .await
        //     .unwrap();

        // if keys_count < 1 {
        //     panic!("Token already used or expired")
        // }

        let access_token = self.token_service.create_access_token("user_id");
        let refresh_token = self.token_service.create_refresh_token();
        let refresh_token_max_age = self.config.jwt.jwt_refresh_token_expire_days * 24 * 3600;

        // self.storage
        //     .create(&session.user_id, &refresh_token, ip_address, refresh_token_max_age)
        //     .await
        //     .unwrap();

        Token {
            access_token: access_token,
            refresh_token: refresh_token,
            refresh_token_max_age: refresh_token_max_age,
        }
    }

    async fn logout(&self, user_id: &str, refresh_token: &str) {
        // self.storage
        //     .delete(user_id, Some(refresh_token))
        //     .await
        //     .unwrap();
    }
}
