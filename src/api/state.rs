use crate::core::config::AppConfig;
use crate::services::oic_service::OICService;

pub struct GlobalState {
    pub config: AppConfig,
    pub oic_service: OICService,
}
