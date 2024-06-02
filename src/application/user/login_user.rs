use std::error::Error;

use uuid::Uuid;

use crate::domain::{password::Password, token::Token, user_repository::UserRepository};

pub trait Login<R> where R: UserRepository + Send + Sync {
    fn new(repository: R) -> Self;
    fn login(&self, email: &str, password: &str) -> Result<Token, Box<dyn Error>>;
}

pub struct  LoginService<R> where R: UserRepository + Send + Sync {
    repository: R,
}

impl<R> Login<R> for LoginService<R> where R: UserRepository + Send + Sync {

    fn new(repository: R) -> Self {
        Self { repository: repository }
    }

    fn login(&self, email: &str, password: &str) -> Result<Token, Box<dyn Error>> {
        let user_found = self.repository.find_by_email(email.to_string());
        
        let user_id = match user_found {
            Ok(value) => value.get_id(),
            Err(err) => return Err(err.into()),
        };
        let credential_found = self.repository.find_credentials_by_user_id(user_id.clone());
        let cred = match credential_found {
            Ok(value) => value,
            Err(err) => return Err(err.into()),
        };
        let roles = self.repository.find_roles_by_user_id(user_id.clone());
        if Password::verify(password, cred.password.as_str()) {
            return match Uuid::parse_str(cred.user_id.as_str()) {
                Ok(value) => Ok(Token::new(value, roles?)),
                Err(err) => Err(err.into())
            }
        }
        Err("Not found".into())
    }
}
