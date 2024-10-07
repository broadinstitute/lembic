use std::fmt::Display;
use crate::error::Error;
use crate::pipe::{LinePipe, Summary};
use crate::runtime::Runtime;
use tokio::io::AsyncBufReadExt;

#[derive(Clone)]
pub(crate) struct S3Location {
    bucket: String,
    key: String,
}

impl S3Location {
    pub(crate) fn new(bucket: String, key: String) -> S3Location { S3Location { bucket, key } }
    pub(crate) fn from_strs(bucket: &str, key: &str) -> S3Location {
        S3Location::new(bucket.to_string(), key.to_string())
    }
    pub(crate) fn bucket(&self) -> &str { &self.bucket }
    pub(crate) fn key(&self) -> &str { &self.key }
}
impl TryFrom<&str> for S3Location {
    type Error = Error;

    fn try_from(uri: &str) -> Result<Self, Self::Error> {
        if let Some(path) = uri.strip_prefix("s3://") {
            let mut parts = path.splitn(2, '/');
            match (parts.next(), parts.next()) {
                (Some(bucket), Some(key)) =>
                    Ok(S3Location { bucket: bucket.to_string(), key: key.to_string() }),
                _ =>
                    Err(Error::from(
                        format!("Invalid S3 URI: Need s3://<bucket>/<key>: '{}'", uri)))
            }
        } else {
            Err(Error::from(format!("Invalid S3 URI: missing `s3://`: '{}'", uri)))
        }
    }
}

impl Display for S3Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "s3://{}/{}", self.bucket, self.key)
    }
}

pub(crate) fn process<P>(runtime: &Runtime, pipe: &P) -> Result<(), Error>
where P: LinePipe{
    runtime.tokio().block_on(async {
        let response =
            runtime.s3_client().get_object()
                .bucket(pipe.location().bucket())
                .key(pipe.location().key())
                .send()
                .await?;
        let mut lines = response.body.into_async_read().lines();
        let mut summary = pipe.new_summary();
        while let Some(line) = lines.next_line().await? {
            summary = summary.next(line)?.summary;
        };
        Ok::<(), Error>(())
    })?;
    Ok(())
}
