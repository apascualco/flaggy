use infrastructure::server;
use dotenv::dotenv;

mod infrastructure;
mod domain;
mod application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    return server::server::run().await;
}
