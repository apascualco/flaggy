use std::{error::Error, time::{Duration, SystemTime, UNIX_EPOCH}};

use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub struct Token {
    id: Uuid,
    roles: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    user_id: String,
    roles: Vec<String>,
    exp: usize,
    iss: String,
}

impl TokenClaims {
    pub fn new(token: String) -> Result<Self, Box<dyn Error>> {
        let validation = Validation::new(Algorithm::HS256);
        let key = DecodingKey::from_secret("any".as_ref());
        let token_decode = decode::<TokenClaims>(&token, &key, &validation)?; 
        Ok(TokenClaims { 
            user_id: token_decode.claims.user_id, 
            roles: token_decode.claims.roles, 
            exp: token_decode.claims.exp, 
            iss: token_decode.claims.iss 
        })
    }

    #[allow(unused)]
    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }

    pub fn get_roles(&self) -> &Vec<String> {
        &self.roles
    }

}

impl Token {
    pub fn new(id: Uuid, roles: Vec<String>) -> Self {
        Token {id, roles}
    }


    #[allow(unused)]
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    #[allow(unused)]
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
