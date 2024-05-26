use uuid::Uuid;

pub struct User {
    id: Uuid,
    email: String
}

impl User {
    pub fn new(email: String) -> User {
        User { id: Uuid::new_v4(), email, }
    }

    pub fn with_id(id: Uuid, email: String) -> User {
        User { id, email }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }
}
