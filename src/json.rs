use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use serde_json::{Map, Value};
use crate::error::Error;
use crate::pipe::{LinePipe, NextSummary, Summary};
use crate::runtime::Runtime;
use crate::s3;
use crate::s3::S3Uri;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum JsonType {
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
}

struct JsonFieldSchema {
    json_type: BTreeMap<JsonType, u64>,
}
struct JsonSchema {
    n_objects: u64,
    fields: Vec<String>,
    field_types: BTreeMap<String, JsonFieldSchema>,
}

impl JsonSchema {
    fn new() -> JsonSchema {
        JsonSchema {
            n_objects: 0,
            fields: Vec::new(),
            field_types: BTreeMap::new(),
        }
    }
    fn add_object(&mut self, value: &Value) -> Result<(), Error> {
        self.n_objects += 1;
        if let Value::Object(map) = value {
            for (key, value) in map {
                let field = key.to_string();
                if !self.fields.contains(&field) {
                    self.fields.push(field.clone());
                }
                let json_type = get_json_type(value);
                let field_schema =
                    self.field_types.entry(field).or_insert(JsonFieldSchema {
                        json_type: BTreeMap::new()
                    });
                let count = field_schema.json_type.entry(json_type).or_insert(0);
                *count += 1;
            }
            Ok(())
        } else {
            Err(Error::from(format!("Expected object, but got {}.", value)))
        }
    }
}

impl Display for JsonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonType::Object => write!(f, "Object"),
            JsonType::Array => write!(f, "Array"),
            JsonType::String => write!(f, "String"),
            JsonType::Number => write!(f, "Number"),
            JsonType::Boolean => write!(f, "Boolean"),
            JsonType::Null => write!(f, "Null"),
        }
    }
}

impl Display for JsonSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Objects: {}", self.n_objects)?;
        for field in &self.fields {
            write!(f, "{}: ", field)?;
            let field_schema = self.field_types.get(field).unwrap();
            for (json_type, count) in &field_schema.json_type {
                if *count > 0 {
                    write!(f, "{}: {}, ", json_type, count)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub(crate) fn print_schema(runtime: &Runtime, s3uri: &S3Uri) -> Result<(), Error> {
    let pipe = JsonSchemaPipe::new(s3uri.clone());
    let schema = s3::process(runtime, &pipe)?;
    println!("{}", schema);
    Ok(())
}

pub(crate) fn print_tabular(runtime: &Runtime, s3uri: &S3Uri, columns: &[String])
    -> Result<(), Error> {
    let columns =
        if columns.is_empty() {
            let schema_pipe = JsonSchemaPipe::new(s3uri.clone());
            let schema = s3::process(runtime, &schema_pipe)?;
            schema.fields
        } else {
            columns.to_vec()
        };
    let pipe = TabularPrinterPipe { s3uri: s3uri.clone(), columns };
    s3::process(runtime, &pipe)?;
    Ok(())
}

fn get_json_type(value: &Value) -> JsonType {
    match value {
        Value::Object(_) => JsonType::Object,
        Value::Array(_) => JsonType::Array,
        Value::String(_) => JsonType::String,
        Value::Number(_) => JsonType::Number,
        Value::Bool(_) => JsonType::Boolean,
        Value::Null => JsonType::Null,
    }
}

struct JsonSchemaPipe {
    s3uri: S3Uri
}

impl JsonSchemaPipe {
    fn new(s3uri: S3Uri) -> JsonSchemaPipe { JsonSchemaPipe { s3uri } }
}

impl Summary for JsonSchema {
    fn next(self, line: String) -> Result<NextSummary<Self>, Error> {
        let value: Value = serde_json::from_str(&line)?;
        let mut summary = self;
        if let Value::Object(_) = &value {
            summary.add_object(&value)?;
        }
        Ok(NextSummary { summary })
    }
}
impl LinePipe for JsonSchemaPipe {
    type Summary = JsonSchema;

    fn s3uri(&self) -> &S3Uri { &self.s3uri }

    fn new_summary(&self) -> JsonSchema { JsonSchema::new() }
}

struct TabularPrinterSummary {
    columns: Vec<String>,
}
struct TabularPrinterPipe {
    s3uri: S3Uri,
    columns: Vec<String>,
}

impl Summary for TabularPrinterSummary {
    fn next(self, line: String) -> Result<NextSummary<Self>, Error> {
        let value: Value = serde_json::from_str(&line)?;
        let summary = self;
        if let Value::Object(map) = &value {
            for column in &summary.columns {
                if let Some(value) = map.get(column) {
                    print!("{}\t", value);
                } else {
                    print!("\t");
                }
            }
            println!();
        }
        Ok(NextSummary { summary })
    }
}

impl LinePipe for TabularPrinterPipe {
    type Summary = TabularPrinterSummary;

    fn s3uri(&self) -> &S3Uri { &self.s3uri }

    fn new_summary(&self) -> TabularPrinterSummary {
        println!("#{}", self.columns.join("\t"));
        TabularPrinterSummary { columns: self.columns.clone() }
    }
}

pub(crate) fn as_json_obj(string: &str) -> Result<Map<String, Value>, Error> {
    let value: Value = serde_json::from_str(string)?;
    if let Value::Object(map) = value {
        Ok(map)
    } else {
        Err(Error::from(format!("Expected JSON object, but got {}", value)))
    }
}

fn field_as_string(value: &Value, key: &str) -> Result<String, Error> {
    if let Value::String(string) = value {
        Ok(string.clone())
    } else {
        Err(Error::from(format!("Expected string field '{}', but got {}", key, value)))
    }
}

pub(crate) fn get_string(map: &Map<String, Value>, key: &str) -> Result<String, Error> {
    if let Some(value) = map.get(key) {
        field_as_string(value, key)
    } else {
        Err(Error::from(format!("Missing field '{}'", key)))
    }
}

pub(crate) fn get_number(map: &Map<String, Value>, key: &str) -> Result<f64, Error> {
    if let Some(value) = map.get(key) {
        if let Value::Number(number) = value {
            let number_f64 = number.as_f64().ok_or_else(|| {
                Error::from(format!("Cannot represent {} as a 64 bit float", number))
            })?;
            Ok(number_f64)
        } else {
            Err(Error::from(
                format!("Expected number field '{}', but got {}", key, value)
            ))
        }
    } else {
        Err(Error::from(format!("Missing field '{}'", key)))
    }
}

pub(crate) fn get_string_fallback(map: &Map<String, Value>, key: &str, key2: &str)
                                  -> Result<String, Error> {
    if let Some(value) = map.get(key).filter(|value| !value.is_null()) {
        field_as_string(value, key)
    } else if let Some(value) = map.get(key2) {
        field_as_string(value, key2)
    } else {
        Err(Error::from(format!("Missing both fields '{}' and '{}'", key, key2)))
    }
}



