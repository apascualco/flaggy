use crate::{domain::{user::User, user_repository::UserRepository}, infrastructure::repository::user_entity::{user::dsl::user, UserEntity}};
use chrono::Utc;
use diesel::{r2d2::{self, ConnectionManager}, MysqlConnection, RunQueryDsl};

pub struct UserRepositoryImpl {
    pub pool: r2d2::Pool<ConnectionManager<MysqlConnection>>
}
impl UserRepository for UserRepositoryImpl {
    fn save(&self, id: uuid::Uuid, email: &str, password: &str) -> User {
        println!("Repository -> id: {}, email: {}, password: {}", id, email, password);
        let mut conn = self.pool.get().expect("Failed to get a connection from the pool");

        let user_entity = UserEntity { 
            id: id.to_string(), 
            email: email.to_string(), 
            is_active: true ,
            created_at: Utc::now().naive_utc(),
            updated_at: None
        };
        diesel::insert_into(user).values(&user_entity).execute(&mut *conn).expect("Err");

        User::with_id(id, email.to_string())
    }
}
