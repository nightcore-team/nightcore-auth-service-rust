use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_db: i64,
    pub redis_password: Option<String>,
}
