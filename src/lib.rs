use std::sync::Arc;

use axum::routing::{Router, get, post};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

pub mod api;
pub mod core;
pub mod domain;
pub mod infra;
pub mod services;
pub mod utils;

use crate::api::endpoints::auth::{
    discord_callback_handler, login_handler, logout_handler, refresh_token_handler,
};
use crate::api::state::GlobalState;
use crate::core::config::AppConfig;
use crate::core::env::load_dotend;
use crate::infra::discord::oauth_provider::DiscordOAuthProvider;
use crate::infra::redis::client::create_redis_client;
use crate::infra::redis::repository::RedisStorageRepository;
use crate::services::jwt_service::JwtTokenService;
use crate::services::oic_service::OICService;
use crate::utils::logging::setup_logging;

async fn setup_application() -> (Router, TcpListener, String) {
    let config = AppConfig::from_env();

    let bind = format!("{}:{}", config.api.api_host, config.api.api_port);

    let redis_client = create_redis_client(&config.redis)
        .await
        .expect("Failed to connect to Redis");

    let oauth_provider = Box::new(DiscordOAuthProvider::new(Arc::new(config.discord.clone())));
    let storage = Box::new(RedisStorageRepository::new(redis_client));
    let token_service = Box::new(JwtTokenService::new());

    let oic_service = OICService::new(oauth_provider, token_service, storage);

    let state = Arc::new(GlobalState {
        config,
        oic_service,
    });

    let auth_router = Router::new()
        .route("/login", get(login_handler))
        .route("/refresh", post(refresh_token_handler))
        .route("/logout", post(logout_handler))
        .route("/discord/callback", get(discord_callback_handler))
        .with_state(state);

    let app: Router = Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/auth", auth_router)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(&bind).await.unwrap();

    (app, listener, bind)
}

pub async fn run_application() {
    setup_logging();
    load_dotend();

    let (app, listener, bind) = setup_application().await;

    info!("Starting application on {}", &bind);

    let _ = axum::serve(listener, app).await;
}
