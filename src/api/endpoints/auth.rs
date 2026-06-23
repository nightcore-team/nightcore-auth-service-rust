use std::sync::Arc;

use axum::{extract::State, response::Redirect};

use crate::api::state::GlobalState;

pub async fn refresh_token_handler() {}

pub async fn logout_handler() {}

pub async fn login_handler(State(state): State<Arc<GlobalState>>) -> Redirect {
    Redirect::temporary(&state.oic_service.oauth_provider.get_authorization_url())
}

pub async fn discord_callback_handler() {}
