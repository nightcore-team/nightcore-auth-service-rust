use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use base64::engine::general_purpose;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

use crate::core::config::JWTConfig;
use crate::domain::entities::JWTPayload;
use crate::domain::exceptions::AuthError;
use crate::domain::interfaces::ITokenService;

fn decode_key(v: &str) -> String {
    let decoded = general_purpose::STANDARD.decode(v).unwrap();
    String::from_utf8(decoded).unwrap()
}

pub struct JwtTokenService {
    config: Arc<JWTConfig>,
}

impl JwtTokenService {
    pub fn new(config: Arc<JWTConfig>) -> Self {
        Self { config }
    }
}

impl ITokenService for JwtTokenService {
    fn create_access_token(&self, user_id: &str) -> Result<String, AuthError> {
        self.sign(json!({"sub": user_id }))
    }

    fn create_refresh_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

    fn sign(&self, payload: serde_json::Value) -> Result<String, AuthError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = JWTPayload {
            iat: now,
            exp: now + (self.config.jwt_access_token_expire_minutes * 60),
            payload,
        };

        let sub = claims.payload.get("sub").and_then(|v| v.as_str()).unwrap_or("?");

        debug!(user_id = %sub, "Signing access token");

        encode(
            &Header::new(Algorithm::from_str(&self.config.jwt_algorithm).unwrap()),
            &claims,
            &EncodingKey::from_rsa_pem(decode_key(&self.config.jwt_private_key).as_bytes())
                .map_err(|e| AuthError::InvalidToken(e.to_string()))?,
        )
        .map_err(|e| AuthError::InvalidToken(e.to_string()))
    }
}
