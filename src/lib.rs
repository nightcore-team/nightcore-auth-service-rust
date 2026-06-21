use tracing::info;

mod domain;
mod infra;
mod utils;

pub fn run_application() {
    utils::logging::setup_logging();
    info!("Application started succesfully.")
}
