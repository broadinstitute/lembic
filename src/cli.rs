use crate::error::Error;

mod commands {
    pub const LIST_BUCKETS: &str = "list-buckets";
    pub const ALL: [&str; 1] = [LIST_BUCKETS];
}
pub(crate) enum Command {
    ListBuckets,
}

pub(crate) fn get_command() -> Result<Command, Error> {
    let mut args = std::env::args();
    let _ = args.next();
    match args.next() {
        Some(arg) => {
            match arg.as_str() {
                "list-buckets" => Ok(Command::ListBuckets),
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
