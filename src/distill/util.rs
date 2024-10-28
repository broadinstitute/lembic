use crate::error::Error;

pub(crate) fn parse_mondo_id(mondo_id: &str) -> Result<u64, Error> {
    let mut parts = mondo_id.split(':');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("MONDO"), Some(id), None) => {
            let id =
                id.parse::<u64>().map_err(|parse_error|
                    Error::wrap("Invalid MONDO ID".to_string(), parse_error)
                )?;
            Ok(id)
        }
        _ => Err(Error::from(format!("Invalid MONDO ID: {}", mondo_id)))
    }
}