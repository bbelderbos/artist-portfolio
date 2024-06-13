pub fn get_images(aws_s3_bucket: &str) -> Result<Vec<String>, String> {
    let mut image_urls = Vec::new();
    // hardcoded for now to keep it simple, but in a real-world scenario
    // you would fetch the image URLs from an S3 bucket
    for i in 1..=10 {
        let image_url = format!("https://{}.s3.amazonaws.com/{}.webp", aws_s3_bucket, i);
        image_urls.push(image_url);
    }
    Ok(image_urls)
}
