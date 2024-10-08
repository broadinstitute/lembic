use crate::error::Error;
use crate::s3::S3Uri;

pub(crate) trait Summary {
    type Current;
    fn next(self, line: String) -> Result<NextSummary<Self>, Error> where Self: Sized;
}

pub(crate) struct NextSummary<S: Summary> {
    pub(crate) summary: S,
    pub(crate) current: S::Current,
}

pub(crate) trait LinePipe {
    type Summary: Summary;
    fn s3uri(&self) -> &S3Uri;
    fn new_summary(&self) -> Self::Summary;
}

