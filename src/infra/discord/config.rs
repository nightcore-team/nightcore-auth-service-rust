use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub discord_auth_client_id: String,
    pub discord_auth_client_secret: String,
    pub discord_auth_redirect_uri: String,
}
