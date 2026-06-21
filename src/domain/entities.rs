use serde::{Deserialize, Serialize};

pub struct Token {
    access_token: String,
    refresh_token: String,
}

pub struct TokenData {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: String,
    scope: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub user_id: String,
    pub ip_address: String,
    pub refresh_token: String,
    pub expires_in: i64,
}
