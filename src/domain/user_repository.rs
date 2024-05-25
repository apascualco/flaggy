use uuid::Uuid;

use crate::domain::user::User;

pub trait UserRepository: Send + Sync {
    fn save(&self, id: Uuid, email: &str, password: &str) -> User;
}
