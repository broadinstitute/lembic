use crate::data;
use crate::data::{get_data_location, Source};
use crate::error::Error;
use crate::s3::S3Uri;
use std::path::PathBuf;

mod commands {
    pub(crate) const LIST_BUCKETS: &str = "list-buckets";
    pub(crate) const PRINT_LINES: &str = "print-lines";
    pub(crate) const PRINT_SCHEMA: &str = "print-schema";
    pub(crate) const PRINT_TABULAR: &str = "print-tabular";
    pub(crate) const LIST_SOURCES: &str = "list-sources";
    pub(crate) const REPORT_STATS: &str = "report-stats";
    pub(crate) const PRINT_TURTLE: &str = "print-turtle";
    pub(crate) const EXPORT_UBKG: &str = "export-ubkg";
    pub(crate) const ALL: [&str; 8] = [
        LIST_BUCKETS,
        PRINT_LINES,
        PRINT_SCHEMA,
        PRINT_TABULAR,
        LIST_SOURCES,
        REPORT_STATS,
        PRINT_TURTLE,
        EXPORT_UBKG,
    ];
}
pub(crate) enum Command {
    ListBuckets,
    PrintLines(S3Uri),
    PrintSchema(S3Uri),
    PrintTabular(S3Uri, Vec<String>),
    ListSources,
    ReportStats(Vec<Source>),
    PrintTurtle(Vec<Source>),
    ExportUbkg(PathBuf, Option<Source>),
}

pub(crate) fn get_command_from_parts<I>(mut parts: I) -> Result<Command, Error>
where
    I: Iterator<Item = String>,
{
    match parts.next() {
        Some(arg) => match arg.as_str() {
            commands::LIST_BUCKETS => Ok(Command::ListBuckets),
            commands::PRINT_LINES => {
                let s3uri = parse_object_argument(parts.next())?;
                Ok(Command::PrintLines(s3uri))
            }
            commands::PRINT_SCHEMA => {
                let s3uri = parse_object_argument(parts.next())?;
                Ok(Command::PrintSchema(s3uri))
            }
            commands::PRINT_TABULAR => {
                let s3uri = parse_object_argument(parts.next())?;
                let columns = parts.collect();
                Ok(Command::PrintTabular(s3uri, columns))
            }
            commands::LIST_SOURCES => Ok(Command::ListSources),
            commands::REPORT_STATS => {
                let source = parse_source_argument(parts.next())?;
                let sources = match source {
                    Some(source) => vec![source],
                    None => data::ALL_SOURCES.to_vec(),
                };
                Ok(Command::ReportStats(sources))
            }
            commands::PRINT_TURTLE => {
                let source = parse_source_argument(parts.next())?;
                let sources = match source {
                    Some(source) => vec![source],
                    None => data::ALL_SOURCES.to_vec(),
                };
                Ok(Command::PrintTurtle(sources))
            }
            commands::EXPORT_UBKG => {
                let path = parse_path(parts.next())?;
                let source = parse_source_argument(parts.next())?;
                Ok(Command::ExportUbkg(path, source))
            }
            _ => Err(Error::from(format!(
                "Unknown command '{}'. {}",
                arg,
                known_commands_are()
            ))),
        },
        None => Err(Error::from(format!(
            "No command provided. {}",
            known_commands_are()
        ))),
    }
}

fn parse_object_argument(arg: Option<String>) -> Result<S3Uri, Error> {
    match arg {
        Some(name) => {
            let location = get_data_location(name.as_str())?;
            Ok(location)
        }
        None => Err(Error::from("No object name provided.")),
    }
}

fn parse_source_argument(arg: Option<String>) -> Result<Option<Source>, Error> {
    match arg {
        Some(name) => Ok(Some(Source::try_from(name.as_str())?)),
        None => Ok(None),
    }
}

fn parse_path(arg: Option<String>) -> Result<PathBuf, Error> {
    match arg {
        Some(name) => Ok(PathBuf::from(name)),
        None => Err(Error::from("No path provided.")),
    }
}

fn known_commands_are() -> String {
    format!("Known commands are '{}'.", commands::ALL.join("', '"))
}
