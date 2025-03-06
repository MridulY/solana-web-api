mod api;
mod solana;
mod models;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Enable logging

    HttpServer::new(|| {
        App::new()
            .configure(api::handlers::config) // Register API routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
