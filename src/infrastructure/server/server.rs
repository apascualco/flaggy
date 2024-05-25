use std::sync::Arc;

use actix_web::{web, App, HttpServer};

use crate::{application::user::create_user::{UserCreator, UserCreatorService}, infrastructure::repository::user_repository::UserRepositoryImpl};

use super::{handler::{health::health_check::health, user::post_user::create_user}, middleware::logger::Logger};

pub async  fn run() -> std::io::Result<()> {

    let user_creator = Arc::new(UserCreatorService::new(UserRepositoryImpl));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger)
            .route("/health", web::get().to(health))
            .service(
                web::resource("/user")
                    .app_data(web::Data::new(user_creator.clone()))
                    .route(web::post().to(create_user::<UserRepositoryImpl>))
                )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

