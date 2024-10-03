use tokio::io::AsyncBufReadExt;
use crate::error::Error;
use crate::runtime::Runtime;
use crate::s3::S3Location;

pub(crate) fn read(runtime: &Runtime, location: &str) -> Result<(), Error> {
    let location = S3Location::try_from(location)?;
    println!("Reading object: {}", location);
    runtime.tokio().block_on(async {
        let response =
            runtime.s3_client().get_object()
                .bucket(location.bucket())
                .key(location.key())
                .send()
                .await?;
        let mut lines = response.body.into_async_read().lines();
        while let Some(line) = lines.next_line().await? {
            println!("{}", line);
        };
        Ok::<(), Error>(())
    })?;
    Ok(())
}