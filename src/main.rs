mod handlers;
mod s3;
mod config;

use axum::{Router, routing::get, extract::Extension};
use config::Config;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    // Initialize configuration
    let config = Arc::new(Config::from_env());

    // Build the application with routes and layers
    let app = Router::new()
        .route("/about", get(handlers::about_handler))
        .route("/portfolio", get(handlers::portfolio_handler))
        .layer(Extension(config));

    // Define the address to run the server on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Log the server start message
    tracing::info!("Listening on {}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

