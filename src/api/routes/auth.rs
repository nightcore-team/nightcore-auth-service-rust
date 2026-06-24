use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    body::Body,
    extract::{Query, State},
    http::{Request, status::StatusCode},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};

use crate::{api::state::GlobalState, domain::interfaces::IOICService};

pub async fn refresh_token_handler() {}

pub async fn logout_handler() {}

pub async fn login_handler(State(state): State<Arc<GlobalState>>) -> Redirect {
    Redirect::temporary(&state.oic_service.oauth_provider.get_authorization_url())
}

pub async fn discord_callback_handler(
    State(state): State<Arc<GlobalState>>,
    Query(params): Query<HashMap<String, String>>,
    request: Request<Body>,
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

    let ip = request
        .headers()
        .get("CF-Connecting-IP")
        .and_then(|v| v.to_str().ok());

    let Some(ip) = ip else {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"detail": "Direct access not allowed"})),
        )
            .into_response();
    };

    let token = state.oic_service.login(&code, &ip).await;

    let cookie = Cookie::build(("refresh_token", token.refresh_token))
        .http_only(true)
        .max_age(time::Duration::seconds(token.refresh_token_max_age))
        .path("/")
        .build();

    let jar = CookieJar::new().add(cookie);

    (jar, Redirect::to(&state.config.api.dashboard_frontend_uri)).into_response()
}
