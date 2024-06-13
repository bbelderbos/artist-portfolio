use askama::Template;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::Html,
};
use crate::Config;
use crate::s3::get_images;
use std::sync::Arc;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    image_url: String,
}

pub async fn about_handler(Extension(config): Extension<Arc<Config>>) -> Result<Html<String>, StatusCode> {
    let image_key = "artist.png";
    let image_url = format!("https://{}.s3.amazonaws.com/{}", config.aws_s3_bucket, image_key);

    let template = AboutTemplate { image_url };
    match template.render() {
        Ok(rendered) => Ok(Html(rendered)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Template)]
#[template(path = "portfolio.html")]
struct PortfolioTemplate {
    image_urls: Vec<String>,
}

pub async fn portfolio_handler(Extension(config): Extension<Arc<Config>>) -> Result<Html<String>, StatusCode> {
    let images = get_images(&config.aws_s3_bucket).unwrap_or_else(|_| vec![]);
    let template = PortfolioTemplate { image_urls: images };

    match template.render() {
        Ok(rendered) => Ok(Html(rendered)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

