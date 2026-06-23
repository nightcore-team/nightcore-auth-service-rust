use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::entities::Token;
use crate::domain::interfaces::{IOAuthProvider, IOICService, IStorageRepository, ITokenService};

pub struct OICService {
    pub oauth_provider: Arc<dyn IOAuthProvider>,
    pub token_service: Arc<dyn ITokenService>,
    pub storage: Arc<dyn IStorageRepository>,
}

impl OICService {
    pub fn new(
        oauth_provider: Arc<dyn IOAuthProvider>,
        token_service: Arc<dyn ITokenService>,
        storage: Arc<dyn IStorageRepository>,
    ) -> Self {
        Self {
            oauth_provider,
            token_service,
            storage,
        }
    }
}

#[async_trait]
impl IOICService for OICService {
    async fn login(&self, code: &str, ip_address: &str) -> Token {
        todo!()
    }

    async fn refresh(&self, refresh_token: &str, ip_address: &str) -> Token {
        todo!()
    }

    async fn logout(&self, user_id: &str, refresh_token: &str) {
        todo!()
    }
}
