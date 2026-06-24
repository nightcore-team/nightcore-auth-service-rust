use std::collections::HashMap;
use std::sync::Arc;

use axum::Json;
use axum::extract::{Query, State};
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::cookie::{Cookie, CookieJar};

use crate::api::state::GlobalState;
use crate::api::utils::auth_error_response;
use crate::domain::exceptions::AuthError;
use crate::domain::interfaces::IOICService;

#[utoipa::path(
    post,
    path = "/auth/refresh",
    responses(
        (status = 200, description = "Tokens refreshed successfully", body = inline(serde_json::Value)),
        (status = 401, description = "Invalid or missing refresh token", body = inline(serde_json::Value)),
        (status = 403, description = "Direct access not allowed", body = inline(serde_json::Value)),
        (status = 502, description = "Upstream error", body = inline(serde_json::Value)),
    ),
)]
pub async fn refresh_token_handler(
    State(state): State<Arc<GlobalState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let refresh_token = jar.get("refresh_token").map(|c| c.value().to_string());

    let Some(refresh_token) = refresh_token else {
        return auth_error_response(AuthError::RefreshTokenNotProvided).into_response();
    };

    let token = match state.oic_service.refresh(&refresh_token).await {
        Ok(token) => token,
        Err(e) => return auth_error_response(e).into_response(),
    };

    let cookie = Cookie::build(("refresh_token", token.refresh_token))
        .http_only(true)
        .max_age(time::Duration::seconds(token.refresh_token_max_age))
        .path("/")
        .build();

    let jar = CookieJar::new().add(cookie);

    (
        jar,
        Json(serde_json::json!({
            "access_token": token.access_token,
            "refresh_token_max_age": token.refresh_token_max_age,
        })),
    )
        .into_response()
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 302, description = "Redirect to dashboard"),
    ),
)]
pub async fn logout_handler(
    State(state): State<Arc<GlobalState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let refresh_token = jar.get("refresh_token").map(|c| c.value().to_string());

    if let Some(refresh_token) = refresh_token {
        let _ = state.oic_service.logout("", &refresh_token).await;
    }

    let cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .max_age(time::Duration::seconds(0))
        .path("/")
        .build();

    let jar = CookieJar::new().add(cookie);

    (jar, Redirect::to(&state.config.api.dashboard_frontend_uri)).into_response()
}
#[utoipa::path(
    get,
    path = "/auth/login",
    responses(
        (status = 302, description = "Redirect to Discord OAuth authorization URL"),
    ),
)]
pub async fn login_handler(State(state): State<Arc<GlobalState>>) -> Redirect {
    Redirect::temporary(&state.oic_service.oauth_provider.get_authorization_url())
}

#[utoipa::path(
    get,
    path = "/auth/discord/callback",
    params(
        ("code" = Option<String>, Query, description = "Authorization code from Discord"),
        ("error" = Option<String>, Query, description = "Error from Discord if user denied access"),
    ),
    responses(
        (status = 302, description = "Redirect to dashboard with refresh_token cookie"),
        (status = 400, description = "Authorization code not found", body = inline(serde_json::Value)),
        (status = 403, description = "Direct access not allowed", body = inline(serde_json::Value)),
        (status = 502, description = "Discord API error", body = inline(serde_json::Value)),
    ),
)]
pub async fn discord_callback_handler(
    State(state): State<Arc<GlobalState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let error = params.get("error");
    let code = params.get("code");

    if let Some(e) = error {
        tracing::error!(error = %e, "OAuth callback returned an error");
        return Redirect::to(&state.config.api.dashboard_frontend_uri).into_response();
    }

    let Some(code) = code else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "detail": "Code is not found in query"
            })),
        )
            .into_response();
    };

    let token = match state.oic_service.login(code).await {
        Ok(token) => token,
        Err(e) => return auth_error_response(e).into_response(),
    };

    let cookie = Cookie::build(("refresh_token", token.refresh_token))
        .http_only(true)
        .max_age(time::Duration::seconds(token.refresh_token_max_age))
        .path("/")
        .build();

    let jar = CookieJar::new().add(cookie);

    (jar, Redirect::to(&state.config.api.dashboard_frontend_uri)).into_response()
}
