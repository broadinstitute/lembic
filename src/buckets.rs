use crate::error::Error;

pub async fn list_buckets() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    let result = s3_client.list_buckets().send().await?;
    match result.buckets {
        None => {
            println!("No buckets found");
        }
        Some(buckets) => {
            for bucket in buckets {
                println!("Bucket: {}", bucket.name().unwrap_or("<no name>"));
            }
        }
    };
    Ok(())
}