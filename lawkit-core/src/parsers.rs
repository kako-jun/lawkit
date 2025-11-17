use anyhow::{anyhow, Result};
use csv::ReaderBuilder;
use serde_json::Value;

// ============================================================================
// PARSER FUNCTIONS - FOR INTERNAL USE ONLY
// ============================================================================
// These functions are public only for CLI and language bindings.
// External users should use the main law() function with file reading.

/// Parse JSON content - FOR INTERNAL USE ONLY
/// External users should read files themselves and use law() function
pub fn parse_json(content: &str) -> Result<Value> {
    serde_json::from_str(content).map_err(|e| anyhow!("JSON parse error: {e}"))
}

/// Parse CSV content - FOR INTERNAL USE ONLY
pub fn parse_csv(content: &str) -> Result<Value> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let headers = reader.headers()?.clone();
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut map = serde_json::Map::new();

        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                map.insert(header.to_string(), Value::String(field.to_string()));
            }
        }

        records.push(Value::Object(map));
    }

    Ok(Value::Array(records))
}

/// Parse YAML content - FOR INTERNAL USE ONLY
pub fn parse_yaml(content: &str) -> Result<Value> {
    serde_yaml::from_str(content).map_err(|e| anyhow!("YAML parse error: {}", e))
}

/// Parse TOML content - FOR INTERNAL USE ONLY
pub fn parse_toml(content: &str) -> Result<Value> {
    let toml_value: toml::Value = content.parse()?;
    toml_to_json_value(toml_value)
}

fn toml_to_json_value(toml_val: toml::Value) -> Result<Value> {
    match toml_val {
        toml::Value::String(s) => Ok(Value::String(s)),
        toml::Value::Integer(i) => Ok(Value::Number(i.into())),
        toml::Value::Float(f) => Ok(Value::Number(
            serde_json::Number::from_f64(f).ok_or_else(|| anyhow!("Invalid float"))?,
        )),
        toml::Value::Boolean(b) => Ok(Value::Bool(b)),
        toml::Value::Array(arr) => {
            let mut json_arr = Vec::new();
            for item in arr {
                json_arr.push(toml_to_json_value(item)?);
            }
            Ok(Value::Array(json_arr))
        }
        toml::Value::Table(table) => {
            let mut json_obj = serde_json::Map::new();
            for (key, value) in table {
                json_obj.insert(key, toml_to_json_value(value)?);
            }
            Ok(Value::Object(json_obj))
        }
        toml::Value::Datetime(dt) => Ok(Value::String(dt.to_string())),
    }
}

/// Parse INI content - FOR INTERNAL USE ONLY
pub fn parse_ini(content: &str) -> Result<Value> {
    let mut result = serde_json::Map::new();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].to_string();
            result.insert(
                current_section.clone(),
                Value::Object(serde_json::Map::new()),
            );
        } else if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_string();
            let value = line[eq_pos + 1..].trim().to_string();

            if current_section.is_empty() {
                result.insert(key, Value::String(value));
            } else if let Some(Value::Object(section)) = result.get_mut(&current_section) {
                section.insert(key, Value::String(value));
            }
        }
    }

    Ok(Value::Object(result))
}

/// Parse XML content - FOR INTERNAL USE ONLY
pub fn parse_xml(content: &str) -> Result<Value> {
    // Simple XML parser - for production use, consider using quick-xml
    // This is a simplified implementation
    Ok(Value::String(format!(
        "XML parsing not fully implemented: {}",
        content.len()
    )))
}
