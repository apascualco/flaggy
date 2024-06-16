use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use diesel::{r2d2::ConnectionManager, MysqlConnection};

use crate::{
    application::{
        feature::create_feature::{FeatureCreator, FeatureCreatorService}, 
        user::{
            create_user::{UserCreator, UserCreatorService},
            login_user::{Login, LoginService}
        },
    }, 
    infrastructure::repository::user_repository::UserRepositoryImpl
};

use super::{handler::{feature::post_feature::create_feature, health::health_check::health, user::{post_login::login_user, post_user::create_user}}, middleware::{logger::Logger, security::Security}};

pub async  fn run() -> std::io::Result<()> {

    let db_connection_pool = establish_connection();
    let user_repository = UserRepositoryImpl{ pool: db_connection_pool };
    let user_creator = Arc::new(UserCreatorService::new(user_repository.clone()));
    let login = Arc::new(LoginService::new(user_repository.clone()));
    let feature_creator = Arc::new(FeatureCreatorService::new(user_repository.clone()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger)
            .route("/health", web::get().to(health))
            .service(
                web::resource("/user")
                    .app_data(web::Data::new(user_creator.clone()))
                    .route(web::post().to(create_user::<UserRepositoryImpl>))
                )
            .service(
                web::resource("/login")
                    .app_data(web::Data::new(login.clone()))
                    .route(web::post().to(login_user::<UserRepositoryImpl>))
                )
            .service(
                web::resource("/feature")
                    .app_data(web::Data::new(feature_creator.clone()))
                    .route(web::post().to(create_feature::<UserRepositoryImpl>))
                    .wrap(Security{ min_role: "USER".to_string() })
                )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn establish_connection() -> r2d2::Pool<ConnectionManager<MysqlConnection>> {
    let host = std::env::var("FLAGGY_DB_HOST")
        .expect("FLAGGY_DB_HOST must be set.");

    let user = std::env::var("FLAGGY_DB_USER")
        .expect("FLAGGY_DB_USER must be set.");

    let password = std::env::var("FLAGGY_DB_PASSWORD")
        .expect("FLAGGY_DB_PASSWORD must be set.");

    let db_name = std::env::var("FLAGGY_DB_NAME")
        .expect("FLAGGY_DB_NAME must be set.");

    let database_url = format!("mysql://{}:{}@{}:3306/{}?charset=utf8&parseTime=True", user, password, host, db_name);

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

