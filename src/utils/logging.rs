use tracing_subscriber;

pub fn setup_logging() {
    tracing_subscriber::fmt::init()
}
