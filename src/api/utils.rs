use axum::Json;
use axum::http::StatusCode;
use tracing::error;

use crate::domain::exceptions::AuthError;

pub fn auth_error_response(err: AuthError) -> (StatusCode, Json<serde_json::Value>) {
    let status = match &err {
        AuthError::SessionNotFound(_)
        | AuthError::RefreshTokenNotProvided
        | AuthError::TokenRevoked => {
            error!(error = %err, "Unauthorized");
            StatusCode::UNAUTHORIZED
        },
        AuthError::AuthorizationCodeNotProvided => {
            error!(error = %err, "Bad request");
            StatusCode::BAD_REQUEST
        },
        AuthError::DiscordAuth(_) | AuthError::Http(_) => {
            error!(error = %err, "Upstream error");
            StatusCode::BAD_GATEWAY
        },
        AuthError::InvalidToken(_) | AuthError::Redis(_) | AuthError::Serde(_) => {
            error!(error = %err, "Internal server error");
            StatusCode::INTERNAL_SERVER_ERROR
        },
    };

    (status, Json(serde_json::json!({"detail": err.to_string()})))
}
