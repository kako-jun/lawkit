//! JSON format tests

use std::process::Command;
use std::fs;
use std::io::Write;

#[test]
fn test_json_array_format() {
    let temp_file = "temp_json_array.json";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, r#"[
        {{"amount": 123.45, "date": "2024-01-01"}},
        {{"amount": 234.56, "date": "2024-01-02"}},
        {{"amount": 345.67, "date": "2024-01-03"}}
    ]"#).expect("Failed to write JSON");
    
    let output = Command::new("cargo")
        .args(["run", "--", "analyze", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    // Cleanup
    let _ = fs::remove_file(temp_file);
    
    // Should either handle JSON or provide meaningful error
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("json") || stderr.contains("format") || stderr.contains("JSON"));
    }
}

#[test]
fn test_json_object_format() {
    let temp_file = "temp_json_object.json";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, r#"{{
        "data": [
            {{"value": 123.45}},
            {{"value": 234.56}},
            {{"value": 345.67}}
        ]
    }}"#).expect("Failed to write JSON");
    
    let output = Command::new("cargo")
        .args(["run", "--", "analyze", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    // Cleanup
    let _ = fs::remove_file(temp_file);
    
    // Should either handle nested JSON or provide meaningful error
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("json") || stderr.contains("format") || stderr.contains("structure"));
    }
}