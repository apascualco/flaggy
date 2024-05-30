use diesel::result::Error;
use uuid::Uuid;

use crate::{domain::user::User, infrastructure::repository::user_credential_entity::UserCredential};

pub trait UserRepository: Send + Sync {
    fn save(&self, id: Uuid, email: &str, password: &str) -> Result<User, Error>;
    fn find_by_email(&self, email: String) -> Result<User, Error>;
    fn find_credentials_by_user_id(&self, id: String) -> Result<UserCredential, Error>;
}

