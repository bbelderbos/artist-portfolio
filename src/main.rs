mod handlers;
mod s3;

use axum::{Router, routing::get, extract::Extension};
use dotenv::dotenv;
use std::net::SocketAddr;
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
        .route("/about", get(handlers::about_handler))
        .route("/portfolio", get(handlers::portfolio_handler))
        .layer(Extension(config.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct Config {
    pub aws_s3_bucket: String,
}
