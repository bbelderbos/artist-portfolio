use rusoto_core::{Region, HttpClient};
use rusoto_credential::StaticProvider;
use rusoto_s3::{S3Client, S3, ListObjectsV2Request};
use crate::config::Config;
use std::sync::Arc;

pub async fn get_images(config: &Arc<Config>) -> Result<Vec<String>, String> {
    let region = config.aws_region.parse().unwrap_or(Region::UsEast1);
    let s3_client = S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        StaticProvider::new_minimal(
            config.aws_access_key_id.clone(),
            config.aws_secret_access_key.clone(),
        ),
        region,
    );

    let list_req = ListObjectsV2Request {
        bucket: config.aws_s3_bucket.clone(),
        ..Default::default()
    };

    match s3_client.list_objects_v2(list_req).await {
        Ok(output) => {
            if let Some(contents) = output.contents {
                let mut image_urls = Vec::new();
                for object in contents {
                    if let Some(key) = object.key {
                        let image_url = format!("https://{}.s3.amazonaws.com/{}", config.aws_s3_bucket, key);
                        image_urls.push(image_url);
                    }
                }
                Ok(image_urls)
            } else {
                Ok(vec![])
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

