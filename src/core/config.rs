use std::env;

pub trait BaseConfig: Sized {
    fn from_env() -> Self;
}

#[derive(Clone)]
pub struct RedisConfig {
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_db: u16,
    pub redis_password: Option<String>,
}

impl BaseConfig for RedisConfig {
    fn from_env() -> Self {
        Self {
            redis_host: env::var("REDIS_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            redis_port: env::var("REDIS_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .expect("REDIS_PORT must be set and parseable as u16"),
            redis_db: env::var("REDIS_DB")
                .ok()
                .and_then(|v| v.parse().ok())
                .expect("REDIS_DB must be set and parseable as u16"),
            redis_password: env::var("REDIS_PASSWORD").ok(),
        }
    }
}

#[derive(Clone)]
pub struct DiscordConfig {
    pub discord_auth_client_id: String,
    pub discord_auth_client_secret: String,
    pub discord_auth_redirect_uri: String,
}

impl BaseConfig for DiscordConfig {
    fn from_env() -> Self {
        Self {
            discord_auth_client_id: env::var("DISCORD_AUTH_CLIENT_ID")
                .expect("DISCORD_AUTH_CLIENT_ID must be set"),
            discord_auth_client_secret: env::var("DISCORD_AUTH_CLIENT_SECRET")
                .expect("DISCORD_AUTH_CLIENT_SECRET must be set"),
            discord_auth_redirect_uri: env::var("DISCORD_AUTH_REDIRECT_URI")
                .expect("DISCORD_AUTH_REDIRECT_URI must be set"),
        }
    }
}

#[derive(Clone)]
pub struct ApiConfig {
    pub api_host: String,
    pub api_port: u16,
    pub dashboard_frontend_uri: String,
}

impl BaseConfig for ApiConfig {
    fn from_env() -> Self {
        Self {
            api_host: env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            api_port: env::var("API_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .expect("API_PORT must be set and parseable as u16"),
            dashboard_frontend_uri: env::var("DASHBOARD_FRONTEND_URI")
                .expect("DASHBOARD_FRONTEND_URI must be set"),
        }
    }
}

#[derive(Clone)]
pub struct JWTConfig {
    pub jwt_public_key: String,
    pub jwt_private_key: String,
    pub jwt_algorithm: String,
    pub jwt_access_token_expire_minutes: u64,
    pub jwt_refresh_token_expire_days: i64,
}

impl BaseConfig for JWTConfig {
    fn from_env() -> Self {
        Self {
            jwt_public_key: env::var("JWT_PUBLIC_KEY").expect("JWT_PUBLIC_KEY must be set"),
            jwt_private_key: env::var("JWT_PRIVATE_KEY").expect("JWT_PRIVATE_KEY must be set"),
            jwt_algorithm: env::var("JWT_ALGORITHM").expect("JWT_ALGORITHM must be set"),
            jwt_access_token_expire_minutes: env::var("JWT_ACCESS_TOKEN_EXPIRE_MINUTES")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .expect("JWT_ACCESS_TOKEN_EXPIRE_MINUTES must be a number"),
            jwt_refresh_token_expire_days: env::var("JWT_REFRESH_TOKEN_EXPIRE_DAYS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .expect("JWT_REFRESH_TOKEN_EXPIRE_DAYS must be a number"),
        }
    }
}

#[derive(Clone)]
pub struct AppConfig {
    pub redis: RedisConfig,
    pub discord: DiscordConfig,
    pub api: ApiConfig,
    pub jwt: JWTConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            redis: RedisConfig::from_env(),
            discord: DiscordConfig::from_env(),
            api: ApiConfig::from_env(),
            jwt: JWTConfig::from_env(),
        }
    }
}
