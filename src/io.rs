use std::fs::File;
use std::path::Path;
use crate::error::Error;

pub(crate) fn create_file<P: AsRef<Path>>(path: P) -> Result<File, Error> {
    File::create(&path).map_err(|error|
        Error::wrap(format!("Failed to create file: {}", path.as_ref().to_string_lossy()),
                    error)
    )
}

pub(crate) fn open_file<P: AsRef<Path>>(path: P) -> Result<File, Error> {
    File::open(&path).map_err(|error|
        Error::wrap(format!("Failed to open file: {}", path.as_ref().to_string_lossy()),
                    error)
    )
}