use crate::domain::interfaces::ITokenService;

pub struct JwtTokenService;

impl JwtTokenService {
    pub fn new() -> Self {
        Self
    }
}

impl ITokenService for JwtTokenService {
    fn create_access_token(&self, user_id: &str) -> String {
        todo!()
    }

    fn create_refresh_token(&self) -> String {
        todo!()
    }
}
