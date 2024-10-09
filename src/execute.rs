use crate::{buckets, data, json, read};
use crate::dsl::Command;
use crate::error::Error;
use crate::runtime::Runtime;

pub(crate) fn execute(runtime: &Runtime, command: &Command) -> Result<(), Error> {
    match command {
        Command::ListBuckets => buckets::list(runtime),
        Command::PrintLines(s3uri) => read::print_lines(runtime, s3uri),
        Command::PrintSchema(s3uri) => { json::print_schema(runtime, s3uri) }
        Command::PrintTabular(s3uri, columns) => {
            json::print_tabular(runtime, s3uri, columns)
        }
        Command::ListSources => {
            data::list_sources();
            Ok(())
        }
    }
}