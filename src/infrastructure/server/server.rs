use actix_web::{App, HttpServer};

use super::{handler::health::health_check::health, middleware::logger::Logger};

pub async  fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().wrap(Logger).service(health)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
