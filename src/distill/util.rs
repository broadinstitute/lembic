use crate::error::Error;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;

pub(crate) fn parse_mondo_id(mondo_id: &str) -> Result<u32, Error> {
    let mut parts = mondo_id.split(':');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("MONDO"), Some(id), None) => {
            let id = id
                .parse::<u32>()
                .map_err(|parse_error| Error::wrap("Invalid MONDO ID".to_string(), parse_error))?;
            Ok(id)
        }
        _ => Err(Error::from(format!("Invalid MONDO ID: {}", mondo_id))),
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

#[derive(Copy, Clone)]
pub(crate) struct OrdF64 {
    pub(crate) value: f64,
}

impl OrdF64 {
    pub(crate) fn new(value: f64) -> OrdF64 { OrdF64 { value } }
}

impl Eq for OrdF64 {}

impl PartialEq<Self> for OrdF64 {
    fn eq(&self, other: &Self) -> bool {
        match (self.value.is_nan(), other.value.is_nan()) {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => self.value == other.value
        }
    }
}

impl PartialOrd<Self> for OrdF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.value.is_nan(), other.value.is_nan()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => {
                if self.value < other.value {
                    Ordering::Less
                } else if self.value > other.value {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

pub(crate) struct PrettyF64 {
    value: f64,
}

impl PrettyF64 {
    pub(crate) fn new(value: f64) -> PrettyF64 { PrettyF64 { value } }
}

impl Display for PrettyF64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let normal = format!("{}", self.value);
        if normal.len() <= 7 {
            write!(f, "{}", normal)
        } else {
            let scientific = format!("{:e}", self.value);
            if scientific.len() < normal.len() {
                write!(f, "{}", scientific)
            } else {
                write!(f, "{}", normal)
            }
        }
    }
}

pub(crate) fn pretty_f64(value: f64) -> PrettyF64 { PrettyF64::new(value) }


