use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Invalid refresh token")]
    RefreshTokenNotProvided,

    #[error("Token has been revoked")]
    TokenRevoked,

    #[error("Authorization code not provided")]
    AuthorizationCodeNotProvided,

    #[error("Discord auth error: {0}")]
    DiscordAuth(String),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}
