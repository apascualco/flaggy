use actix_web::{App, HttpServer};

use crate::infrastructure::endpoints::health::health_check::health;


pub async  fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(health)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
