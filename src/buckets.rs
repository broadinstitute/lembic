use crate::error::Error;
use crate::runtime::Runtime;

pub(crate) fn list(runtime: &Runtime) -> Result<(), Error> {
    let result =
        runtime.tokio().block_on(async {
            runtime.s3_client().list_buckets().send().await
        })?;
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