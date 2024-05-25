use crate::domain::{user::User, user_repository::UserRepository};

pub struct UserRepositoryImpl;
impl UserRepository for UserRepositoryImpl {
    fn save(&self, id: uuid::Uuid, email: &str, password: &str) -> User {
        println!("Repository -> id: {}, email: {}, password: {}", id, email, password);
        User::with_id(id, email.to_string())
    }
}
