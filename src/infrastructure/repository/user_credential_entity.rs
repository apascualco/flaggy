use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::Insertable, table};

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = credential)]
pub struct UserCredential {
    pub user_id: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

table! {
    credential (user_id) {
        user_id -> VarChar,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
