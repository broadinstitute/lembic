use crate::{buckets, read};
use crate::dsl::Command;
use crate::error::Error;
use crate::runtime::Runtime;

pub(crate) fn execute(runtime: &Runtime, command: &Command) -> Result<(), Error> {
    match command {
        Command::ListBuckets => buckets::list(runtime),
        Command::PrintLines(location) => read::print_lines(runtime, location)
    }
}