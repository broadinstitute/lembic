use crate::error::Error;

pub(crate) fn parse_mondo_id(mondo_id: &str) -> Result<u32, Error> {
    let mut parts = mondo_id.split(':');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("MONDO"), Some(id), None) => {
            let id =
                id.parse::<u32>().map_err(|parse_error|
                    Error::wrap("Invalid MONDO ID".to_string(), parse_error)
                )?;
            Ok(id)
        }
        _ => Err(Error::from(format!("Invalid MONDO ID: {}", mondo_id)))
    }
}

pub(crate) fn clean_up_label(label: &str) -> String {
    let mut string = String::new();
    let mut chars = label.chars();
    while let Some(char) = chars.next() {
        if char == '%' {
            let hex1 = chars.next();
            let hex2 = chars.next();
            if let (Some(hex1), Some(hex2)) = (hex1, hex2) {
                let hex = format!("{}{}", hex1, hex2);
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    string.push(byte as char);
                } else {
                    string.push('%');
                    string.push(hex1);
                    string.push(hex2);
                }
            } else {
                string.push('%');
                if let Some(hex1) = hex1 {
                    string.push(hex1);
                }
            }
        } else if char == '+' || char == '_' {
            string.push(' ');
        } else {
            string.push(char);
        }
    }
    string
}