use application::authentication::services::register_user_service::RegisterUserService;
use crate::application;
use crate::application::authentication::services::authenticate_user_service::AuthenticateUserService;

pub struct ServiceContainer {
    pub authenticate_user_service: AuthenticateUserService,
    pub register_user_service: RegisterUserService
}

impl ServiceContainer {
    pub fn new() -> Self {
        return ServiceContainer {
            authenticate_user_service: AuthenticateUserService::new(),
            register_user_service: RegisterUserService::new(),
        }
    }
}
