use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::api::routes::health::health_handler,
    crate::api::routes::auth::login_handler,
    crate::api::routes::auth::discord_callback_handler,
    crate::api::routes::auth::refresh_token_handler,
    crate::api::routes::auth::logout_handler,
))]
pub struct ApiDoc;
