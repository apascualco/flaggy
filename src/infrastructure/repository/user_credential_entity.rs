use diesel::{deserialize::Queryable, table};

#[derive(Queryable, Debug)]
pub struct UserCredential {
    pub id: i32,
    pub password: String,
}

table! {
    credentials (id) {
        id -> Integer,
        password -> Text,
    }
}
