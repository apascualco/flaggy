use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::Insertable, table};

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = user)]
pub struct UserEntity {
    pub id: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

table! {
    user (id) {
        id -> VarChar,
        email -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
