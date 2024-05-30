use bcrypt::{hash_with_salt, verify, BcryptError};
use rand::{Rng, thread_rng};

pub struct Password<'a> {
    value: &'a str,
}

impl<'a> Password<'a> {
    pub fn new(value: &'a str) -> Password {
        Password { value }
    } 

    fn get_value(&self) -> &'a str {
        self.value
    }

    fn salt() -> [u8; 16] {
        let mut rng = thread_rng();
        let mut salt = [0; 16];
        rng.fill(&mut salt);
        salt
    }

    pub fn hash(&self) -> Result<String, BcryptError> {
        let cost = 14;
        let hash = hash_with_salt(self.value, cost, Password::salt())?;
        return Ok(hash.to_string());
    }

    pub fn verify(password: &str, hash: &str) -> bool { 
        match verify(password, hash) {
            Ok(true) => true,
            Ok(false) => false,
            Err(_) => false
        }
    }
}

