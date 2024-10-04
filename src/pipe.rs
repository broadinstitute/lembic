use crate::error::Error;
use crate::s3::S3Location;

pub(crate) trait Status {
    type Current;
    type Summary;
    fn next(self, line: String) -> Result<Self, Error>;
    fn current(&self) -> Self::Current;
    fn summary(&self) -> Self::Summary;
}
pub(crate) trait LinePipe {
    type S: Status;
    fn location(&self) -> &S3Location;
    fn new_status(&self) -> Self::S;
}
