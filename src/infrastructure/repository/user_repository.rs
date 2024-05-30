use crate::{domain::{user::User, user_repository::UserRepository}, infrastructure::repository::{user_credential_entity::{credential::dsl::{credential, user_id}, UserCredential}, user_entity::{user::dsl::{email, user}, UserEntity}}};
use chrono::Utc;
use diesel::{r2d2::{self, ConnectionManager}, result::Error, Connection, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::prelude::*;

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pub pool: r2d2::Pool<ConnectionManager<MysqlConnection>>
}
impl UserRepository for UserRepositoryImpl {
    fn save(&self, id: uuid::Uuid, user_email: &str, password: &str) -> Result<User, Error> {
        let mut conn = self.pool.get().expect("Failed to get a connection from the pool");

        conn.transaction::<_, Error, _>(|conn| {


            let user_entity = UserEntity { 
                id: id.to_string(), 
                email: user_email.to_string(), 
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

        
            Ok(User::with_id(id, user_email.to_string()))
        })
    }

    fn find_by_email(&self, user_email: String) -> Result<User, Error> {
        let mut conn = self.pool.get().expect("Failed to get a connection from the pool");
        let user_entity = user.filter(email.eq(user_email)).first::<UserEntity>(&mut conn)?;
        Ok(User::with_id(user_entity.id.parse().unwrap(), user_entity.email))
    }

    fn find_credentials_by_user_id(&self, id: String) -> Result<UserCredential, Error> {
        let mut conn = self.pool.get().expect("Failed to get a connection from the pool");
        let cred = credential.filter(user_id.eq(id)).first::<UserCredential>(&mut conn)?;
        Ok(cred)
    }
}
