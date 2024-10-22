use aws_sdk_s3::error::SdkError;
use std::fmt::{Debug, Display, Formatter};

pub struct Error {
    message: String,
    source: Option<Box<dyn std::error::Error>>,
}

impl Error {
    fn new(message: String, source: Option<Box<dyn std::error::Error>>) -> Error {
        Error { message, source }
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        let mut source = self.source();
        while let Some(e) = source {
            write!(f, ": {}", e)?;
            source = e.source();
        }
        Ok(())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::new(message, None)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::new(message.to_string(), None)
    }
}

impl<E: std::error::Error + 'static> From<SdkError<E>> for Error {
    fn from(error: SdkError<E>) -> Self {
        Error::new("AWS SDK error".to_string(), Some(Box::new(error)))
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new("I/O error".to_string(), Some(Box::new(error)))
    }
}

impl From<aws_sdk_s3::primitives::ByteStreamError> for Error {
    fn from(error: aws_sdk_s3::primitives::ByteStreamError) -> Self {
        Error::new("Byte stream error".to_string(), Some(Box::new(error)))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::new("JSON error".to_string(), Some(Box::new(error)))
    }
}

impl From<penyu::error::PenyuError> for Error {
    fn from(error: penyu::error::PenyuError) -> Self {
        Error::new("Penyu error".to_string(), Some(Box::new(error)))
    }
}