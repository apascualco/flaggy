use actix_web::{web, App, HttpServer};

use super::{handler::{health::health_check::health, user::post_user::create_user}, middleware::logger::Logger};

pub async  fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger)
            .route("/health", web::get().to(health))
            .route("/user", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
