use crate::error::Error;

mod error;
mod buckets;
mod cli;
mod read;
mod runtime;
mod s3;
mod pipe;
mod dsl;
mod execute;
mod data;

fn main() -> Result<(), Error> {
    let runtime = runtime::Runtime::new()?;
    let command = cli::get_command_cli()?;
    execute::execute(&runtime, &command)
}


