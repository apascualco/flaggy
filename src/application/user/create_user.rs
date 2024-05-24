use crate::domain::user::User;

pub trait UserCreator: Send + Sync {
    fn create_user(&self, email: &str, password: &str) -> User;
}

pub struct UserCreatorService;
impl UserCreator for UserCreatorService {
   fn create_user(&self, email: &str, password: &str) -> User {
        println!("{} and password {}", email, password);
        User::new(email.to_string())
    }
}
