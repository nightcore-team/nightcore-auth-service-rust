use async_trait::async_trait;

use crate::domain::entities::Token;
use crate::domain::interfaces::{IOAuthProvider, IOICService, IStorageRepository, ITokenService};

pub struct OICService {
    oauth_provider: Box<dyn IOAuthProvider>,
    token_service: Box<dyn ITokenService>,
    storage: Box<dyn IStorageRepository>,
}

impl OICService {
    pub fn new(
        oauth_provider: Box<dyn IOAuthProvider>,
        token_service: Box<dyn ITokenService>,
        storage: Box<dyn IStorageRepository>,
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
