pub struct AuthenticateUserService {}

impl AuthenticateUserService {
    pub fn new() -> AuthenticateUserService {
        return AuthenticateUserService {};
    }

    pub fn authenticate_user(&self) -> () {
        println!("Authenticate!");
    }
}