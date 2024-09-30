use std::env;
use crate::buckets::list_buckets;
use crate::error::Error;

mod error;
mod buckets;

mod commands {
    pub const LIST_BUCKETS: &str = "list-buckets";
    pub const ALL: [&str; 1] = [LIST_BUCKETS];
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut args = env::args();
    let _ = args.next();
    match args.next() {
        Some(arg) => {
            match arg.as_str() {
                commands::LIST_BUCKETS => list_buckets().await,
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

