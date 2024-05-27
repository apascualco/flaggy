use crate::{domain::{user::User, user_repository::UserRepository}, infrastructure::repository::{user_credential_entity::{credential::dsl::credential, UserCredential}, user_entity::{user::dsl::user, UserEntity}}};
use chrono::Utc;
use diesel::{r2d2::{self, ConnectionManager}, result::Error, Connection, MysqlConnection, RunQueryDsl};

pub struct UserRepositoryImpl {
    pub pool: r2d2::Pool<ConnectionManager<MysqlConnection>>
}
impl UserRepository for UserRepositoryImpl {
    fn save(&self, id: uuid::Uuid, email: &str, password: &str) -> Result<User, Error> {
        println!("Repository -> id: {}, email: {}, password: {}", id, email, password);
        let mut conn = self.pool.get().expect("Failed to get a connection from the pool");

        conn.transaction::<_, Error, _>(|conn| {


            let user_entity = UserEntity { 
                id: id.to_string(), 
                email: email.to_string(), 
                is_active: true ,
                created_at: Utc::now().naive_utc(),
                updated_at: None
            };
            let credentials_entity = UserCredential {
                user_id: id.to_string(),
                password: password.to_string(),
                created_at: Utc::now().naive_utc(),
                updated_at: None
            };

            diesel::insert_into(user).values(&user_entity).execute(&mut *conn)?;
            diesel::insert_into(credential).values(&credentials_entity).execute(&mut *conn)?;

        
            Ok(User::with_id(id, email.to_string()))
        })
    }
}
