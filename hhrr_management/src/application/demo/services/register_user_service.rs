pub struct RegisterUserService {}

impl RegisterUserService {
    pub fn new() -> RegisterUserService {
        return RegisterUserService {};
    }

    pub fn register_user(&self) -> () {
        println!("Register!");
    }
}