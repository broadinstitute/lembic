use crate::cli::Command;
use crate::error::Error;

mod error;
mod buckets;
mod cli;
mod read;
mod runtime;
mod s3;

fn main() -> Result<(), Error> {
    let runtime = runtime::Runtime::new()?;
    match cli::get_command() {
        Ok(command) => {
            match command {
                Command::ListBuckets => buckets::list(&runtime),
                Command::ReadObject(name) => read::read(&runtime, &name)
            }
        }
        Err(error) => Err(error)
    }
}


