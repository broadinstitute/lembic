use crate::buckets::list_buckets;
use crate::error::Error;

mod error;
mod buckets;
mod cli;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match cli::get_command() {
        Ok(command) => {
            match command {
                cli::Command::ListBuckets => list_buckets().await
            }
        }
        Err(e) => Err(e)
    }
}


