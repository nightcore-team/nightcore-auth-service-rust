use crate::domain::exceptions::AuthError;
use axum::{Json, http::StatusCode};

pub fn auth_error_response(err: AuthError) -> (StatusCode, Json<serde_json::Value>) {
    let status = match err {
        AuthError::SessionNotFound(_)
        | AuthError::RefreshTokenNotProvided
        | AuthError::TokenRevoked => StatusCode::UNAUTHORIZED,
        AuthError::AuthorizationCodeNotProvided => StatusCode::BAD_REQUEST,
        AuthError::DiscordAuth(_) | AuthError::Http(_) => StatusCode::BAD_GATEWAY,
        AuthError::InvalidToken(_) | AuthError::Redis(_) | AuthError::Serde(_) => {
            StatusCode::INTERNAL_SERVER_ERROR
        },
    };

    (status, Json(serde_json::json!({"detail": err.to_string()})))
}
