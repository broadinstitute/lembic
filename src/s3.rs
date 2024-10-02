use std::fmt::Display;
use crate::error::Error;

pub(crate) struct S3Location {
    bucket: String,
    key: String,
}

impl S3Location {
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