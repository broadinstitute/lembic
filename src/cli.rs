use crate::dsl::{get_command_from_parts, Command};
use crate::error::Error;


pub(crate) fn get_command_cli() -> Result<Command, Error> {
    let mut args = std::env::args();
    let _ = args.next();
    get_command_from_parts(args)
}
