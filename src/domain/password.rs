use bcrypt::{hash_with_salt, verify, BcryptError};

pub struct Password<'a> {
    value: &'a str,
}

impl<'a> Password<'a> {
    fn new(value: &'a str) -> Password {
        Password { value }
    } 

    fn get_value(&self) -> &'a str {
        self.value
    }

    fn hash(&self) -> Result<String, BcryptError> {
        let salt: [u8; 16] = [0; 16];
        let cost = 14;
        let hash = hash_with_salt(self.value, cost, salt)?;
        return Ok(hash.to_string());
    }

    fn verify(password: &str, hash: &str) -> bool { 
        match verify(password, hash) {
            Ok(true) => true,
            Ok(false) => false,
            Err(_) => false
        }
    }
}

