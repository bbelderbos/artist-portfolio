use std::collections::HashMap;
use std::error::Error;

pub fn get_images(aws_s3_bucket: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut images = HashMap::new();
    // Hardcoded for now to keep it simple, but in a real-world scenario
    // you would fetch the image URLs from an S3 bucket
    for i in 1..=10 {
        let full_image = format!("https://{}.s3.amazonaws.com/{}.webp", aws_s3_bucket, i);
        let thumbnail = full_image.replace(".webp", "_thumb.png");
        images.insert(full_image, thumbnail);
    }
    Ok(images)
}
