use crate::data::get_data_location;
use crate::error::Error;
use crate::s3::S3Location;

mod commands {
    pub(crate) const LIST_BUCKETS: &str = "list-buckets";
    pub(crate) const PRINT_LINES: &str = "print-lines";
    pub(crate) const ALL: [&str; 2] = [LIST_BUCKETS, PRINT_LINES];
}
pub(crate) enum Command {
    ListBuckets,
    PrintLines(S3Location),
}

pub(crate) fn get_command_from_parts<I>(mut parts: I) -> Result<Command, Error>
where I: Iterator<Item=String> {
    match parts.next() {
        Some(arg) => {
            match arg.as_str() {
                commands::LIST_BUCKETS => Ok(Command::ListBuckets),
                commands::PRINT_LINES => {
                    match parts.next() {
                        Some(name) => {
                            let location = get_data_location(name.as_str())?;
                            Ok(Command::PrintLines(location))
                        },
                        None => Err(Error::from("No object name provided."))
                    }
                }
                _ => Err(Error::from(
                    format!("Unknown command '{}'. {}", arg, known_commands_are())
                ))
            }
        }
        None => Err(Error::from(format!("No command provided. {}", known_commands_are())))
    }
}

fn known_commands_are() -> String {
    format!("Known commands are '{}'.", commands::ALL.join("', '"))
}
