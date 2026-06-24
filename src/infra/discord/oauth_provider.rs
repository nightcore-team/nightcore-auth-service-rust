use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;

use crate::{
    core::config::DiscordConfig,
    domain::{
        entities::{DiscordUser, RequestData, TokenData},
        exceptions::AuthError,
        interfaces::IOAuthProvider,
    },
};

pub struct DiscordOAuthProvider {
    config: Arc<DiscordConfig>,
    client: Client,
}

impl DiscordOAuthProvider {
    pub fn new(config: Arc<DiscordConfig>) -> Self {
        Self {
            config,
            client: Client::new(),
        }
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

    fn get_request_data(&self, code: String) -> RequestData {
        RequestData {
            client_id: self.config.discord_auth_client_id.clone(),
            client_secret: self.config.discord_auth_client_secret.clone(),
            grant_type: String::from("authorization_code"),
            code: String::from(code),
            redirect_uri: self.config.discord_auth_redirect_uri.clone(),
        }
    }

    async fn exchange_code(&self, code: String) -> Result<TokenData, AuthError> {
        let response = self
            .client
            .post("https://discord.com/api/oauth2/token")
            .form(&self.get_request_data(code))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            return Err(AuthError::DiscordAuth(format!(
                "Discord API error {status}: {body}"
            )));
        }

        let token_data: TokenData = response.json().await?;
        Ok(token_data)
    }

    async fn get_user_info(&self, token_data: TokenData) -> Result<DiscordUser, AuthError> {
        let response = self
            .client
            .get("https://discord.com/api/users/@me")
            .bearer_auth(&token_data.access_token)
            .send()
            .await
            .map_err(|e| AuthError::DiscordAuth(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            return Err(AuthError::DiscordAuth(format!(
                "Discord API error {status}: {body}"
            )));
        }

        let user: DiscordUser = response.json().await?;
        Ok(user)
    }
}
