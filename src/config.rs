use dotenv::dotenv;
use std::env;

pub struct Config {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_s3_bucket: String,
    pub aws_region: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            aws_access_key_id: env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not set"),
            aws_secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not set"),
            aws_s3_bucket: env::var("AWS_S3_BUCKET").expect("AWS_S3_BUCKET not set"),
            aws_region: env::var("AWS_REGION").expect("AWS_REGION not set"),
        }
    }
}

