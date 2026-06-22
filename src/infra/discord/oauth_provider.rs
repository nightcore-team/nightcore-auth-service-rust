use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;

use crate::{
    core::config::DiscordConfig,
    domain::{
        entities::{RequestData, TokenData},
        exceptions::AuthError,
        interfaces::IOAuthProvider,
    },
};

pub struct DiscordOAuthProvider {
    config: Arc<DiscordConfig>,
}

impl DiscordOAuthProvider {
    pub fn new(config: Arc<DiscordConfig>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl IOAuthProvider for DiscordOAuthProvider {
    fn get_authorization_url(&self) -> String {
        format!(
            "https://discord.com/oauth2/authorize\
             ?client_id={}\
             &redirect_uri={}\
             &response_type=code\
             &scope=identify",
            self.config.discord_auth_client_id, self.config.discord_auth_redirect_uri,
        )
    }

    fn get_request_data(&self, code: &str) -> RequestData {
        RequestData {
            client_id: self.config.discord_auth_client_id.clone(),
            client_secret: self.config.discord_auth_client_secret.clone(),
            grant_type: String::from("authorization_code"),
            code: String::from(code),
            redirect_uri: self.config.discord_auth_redirect_uri.clone(),
        }
    }

    async fn exchange_code(&self, code: Option<&str>) -> Result<TokenData, AuthError> {
        let code = code.ok_or(AuthError::AuthorizationCodeNotProvided)?;

        let client = Client::new();
        let response = client
            .post("https://discord.com/api/oauth2/token")
            .form(&self.get_request_data(code))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(AuthError::DiscordAuth(format!(
                "Discord API error {status}: {body}"
            )));
        }

        let token_data: TokenData = serde_json::from_str(&body)?;
        Ok(token_data)
    }

    async fn get_user_info(&self, token_data: &TokenData) -> Result<String, AuthError> {
        todo!()
    }
}
