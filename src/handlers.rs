use askama::Template;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::Html,
};
use crate::Config;
use crate::s3::get_images;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    image_url: String,
    current_page: &'static str,
}

pub async fn about_handler(Extension(config): Extension<Arc<Config>>) -> Result<Html<String>, StatusCode> {
    let image_key = "artist.png";
    let image_url = format!("https://{}.s3.amazonaws.com/{}", config.aws_s3_bucket, image_key);

    let template = AboutTemplate { image_url, current_page: "about" };
    match template.render() {
        Ok(rendered) => Ok(Html(rendered)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Template)]
#[template(path = "portfolio.html")]
struct PortfolioTemplate {
    images: Vec<(String, String)>,
    current_page: &'static str,
}

pub async fn portfolio_handler(Extension(config): Extension<Arc<Config>>) -> Result<Html<String>, StatusCode> {
    let images = get_images(&config.aws_s3_bucket).unwrap_or_else(|_| HashMap::new());
    // cannot get the template to work with a HashMap directly, so convert to a Vec of tuples
    let images: Vec<(String, String)> = images.into_iter().collect();
    let template = PortfolioTemplate { images, current_page: "portfolio"  };

    match template.render() {
        Ok(rendered) => Ok(Html(rendered)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

