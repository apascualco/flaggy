use uuid::Uuid;

use crate::domain::{password::Password, user::User, user_repository::UserRepository};

pub trait UserCreator<R> where R: UserRepository + Send + Sync {
    fn new(repository: R) -> Self;
    fn create_user(&self, email: &str, password: &str) -> User;
}

pub struct UserCreatorService<R> where R: UserRepository + Send + Sync {
    repository: R,
}
impl<R> UserCreator<R> for UserCreatorService<R> where R: UserRepository + Send + Sync {

    fn new(repository: R) -> Self {
        Self { repository: repository }
    }

    fn create_user(&self, email: &str, password: &str) -> User {
        let password = Password::new(password);
        match password.hash() {
            Ok(value) => {
                let uuid = Uuid::new_v4();
                return self.repository.save(uuid, email, value.as_str());
            } 
            Err(error) => println!("Error: {}", error),
        }
        User::new(email.to_string())
    }

}
