mod handlers;
mod s3;

use axum::{Router, routing::get, extract::Extension};
use dotenv::dotenv;
use tower_http::services::ServeDir;
use tracing_subscriber;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Initialize configuration
    let aws_s3_bucket = std::env::var("AWS_S3_BUCKET").expect("AWS_S3_BUCKET must be set");
    let config = Arc::new(Config { aws_s3_bucket });

    let app = Router::new()
        .route("/", get(handlers::about_handler))
        .route("/portfolio", get(handlers::portfolio_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer(Extension(config.clone()));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string());
    let bind_addr = format!("{}:{}", addr, port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
pub struct Config {
    pub aws_s3_bucket: String,
}
