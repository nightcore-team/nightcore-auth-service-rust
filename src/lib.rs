use tracing::info;

pub mod core;
pub mod domain;
pub mod infra;
pub mod services;
pub mod utils;

use crate::core::config::AppConfig;
use crate::core::env::load_dotend;
use crate::infra::redis::client::create_redis_client;
use crate::utils::logging::setup_logging;

pub async fn run_application() {
    setup_logging();
    load_dotend();

    let _ = AppConfig::from_env();
    // let _redis_client = create_redis_client(&config.redis)
    //     .await
    //     .expect("Failed to connect to Redis");

    info!("Application started successfully.");
}
