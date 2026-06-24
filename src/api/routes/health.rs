#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = String),
    ),
)]
pub async fn health_handler() -> &'static str {
    "OK"
}
