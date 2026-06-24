use std::{
    str::FromStr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    core::config::JWTConfig,
    domain::{entities::JWTPayload, exceptions::AuthError, interfaces::ITokenService},
};
use base64::{Engine as _, engine::general_purpose};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use rand::{RngExt, distr::Alphanumeric};
use serde_json::json;
use uuid::Uuid;

pub struct JwtTokenService {
    config: Arc<JWTConfig>,
}

impl JwtTokenService {
    pub fn new(config: Arc<JWTConfig>) -> Self {
        Self { config: config }
    }
}

impl ITokenService for JwtTokenService {
    fn decode_key(&self, v: &str) -> String {
        let decoded = general_purpose::STANDARD.decode(v).unwrap();
        return String::from_utf8(decoded).unwrap();
    }
    fn create_access_token(&self, user_id: &str) -> String {
        return self.sign(json!({"sub": user_id })).unwrap();
    }

    fn create_refresh_token(&self) -> String {
        let random_bytes: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        Uuid::new_v5(&Uuid::NAMESPACE_DNS, random_bytes.as_bytes()).to_string()
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

        encode(
            &Header::new(Algorithm::from_str(&self.config.jwt_algorithm).unwrap()),
            &claims,
            &EncodingKey::from_rsa_pem(self.decode_key(&self.config.jwt_private_key).as_bytes())
                .map_err(|e| AuthError::InvalidToken(e.to_string()))?,
        )
        .map_err(|e| AuthError::InvalidToken(e.to_string()))
    }
}
