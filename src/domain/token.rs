use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, EncodingKey, Header};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub struct Token {
    id: Uuid,
    roles: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    user_id: String,
    roles: Vec<String>,
    exp: usize,
    iss: String,
}

impl Token {
    pub fn new(id: Uuid, roles: Vec<String>) -> Self {
        Token {id, roles}
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_roles(&self) -> &Vec<String> {
        &self.roles
    }

    pub fn build(&self) -> String {
        let minuts = 360;
        let exp = SystemTime::now().checked_add(Duration::from_secs(minuts * 60))
            .unwrap_or(UNIX_EPOCH)
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize;
        let claims = TokenClaims {
            user_id: self.id.to_string(),
            roles: self.roles.clone(),
            exp,
            iss: "flagger".to_string(),
        };
        let key = EncodingKey::from_secret("any".as_ref());
        match jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, &key) {
            Ok(value) => value,
            Err(_) => String::from("")
        }
    }
}
