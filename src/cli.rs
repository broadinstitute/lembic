use crate::error::Error;

mod commands {
    pub(crate) const LIST_BUCKETS: &str = "list-buckets";
    pub(crate) const PRINT_LINES: &str = "print-lines";
    pub(crate) const ALL: [&str; 2] = [LIST_BUCKETS, PRINT_LINES];
}
pub(crate) enum Command {
    ListBuckets,
    PrintLines(String),
}

pub(crate) fn get_command() -> Result<Command, Error> {
    let mut args = std::env::args();
    let _ = args.next();
    match args.next() {
        Some(arg) => {
            match arg.as_str() {
                commands::LIST_BUCKETS => Ok(Command::ListBuckets),
                commands::PRINT_LINES => {
                    match args.next() {
                        Some(name) => Ok(Command::PrintLines(name)),
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
