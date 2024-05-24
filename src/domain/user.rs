use uuid::Uuid;

pub struct User {
    id: Uuid,
    email: String
}

impl User {
    pub fn new(email: String) -> User {
        User { id: Uuid::new_v4(), email, }
    }
}
