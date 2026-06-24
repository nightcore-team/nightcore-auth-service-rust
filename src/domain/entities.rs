use serde::{Deserialize, Serialize};

pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub refresh_token_max_age: i64,
}

#[derive(Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Serialize)]
pub struct RequestData {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub user_id: String,
    pub ip_address: String,
}

#[derive(Deserialize)]
pub struct DiscordUser {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct JWTPayload {
    pub iat: u64,
    pub exp: u64,

    #[serde(flatten)]
    pub payload: serde_json::Value,
}
