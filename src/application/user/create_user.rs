use crate::domain::{password::Password, user::User};

pub trait UserCreator: Send + Sync {
    fn create_user(&self, email: &str, password: &str) -> User;
}

pub struct UserCreatorService;
impl UserCreator for UserCreatorService {
    fn create_user(&self, email: &str, password: &str) -> User {
        let password = Password::new(password);
        match password.hash() {
            Ok(value) => println!("{} anpassword {}", email, value),
            Err(error) => println!("Error: {}", error),
        }
        
        
        User::new(email.to_string())
    }
}
