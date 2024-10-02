use crate::error::Error;
use crate::runtime::Runtime;
use crate::s3::S3Location;

pub(crate) fn read(runtime: &Runtime, location: &str) -> Result<(), Error> {
    let location = S3Location::try_from(location)?;
    println!("Reading object: {}", location);
    let data =
        runtime.tokio().block_on(async {
            let response =
                runtime.s3_client().get_object()
                    .bucket(location.bucket())
                    .key(location.key())
                    .send()
                    .await?;
            let bytes = response.body.collect().await?.to_vec();
            let string = String::from_utf8_lossy(&bytes).to_string();
            Ok::<String, Error>(string)
        })?;
    println!("{}", data);
    Ok(())
}