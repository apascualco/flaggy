use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::Insertable, table};

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = user_role)]
pub struct UserRoleEnttty {
    pub role: String,
    pub user_id: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

table! {
    user_role (role, user_id) {
        role -> VarChar,
        user_id -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
