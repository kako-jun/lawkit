//! Excel format tests

use std::process::Command;

#[test]
fn test_excel_file_detection() {
    let test_file = "tests/fixtures/sample_data.xlsx";

    let output = Command::new("cargo")
        .args(["run", "--", "analyze", test_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    // Should either succeed or provide meaningful error about Excel support
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Should mention Excel or xlsx in error message if not supported
        assert!(stderr.contains("xlsx") || stderr.contains("Excel") || stderr.contains("format"));
    }
}

#[test]
fn test_ods_file_detection() {
    let test_file = "tests/fixtures/sample_data.ods";

    let output = Command::new("cargo")
        .args(["run", "--", "analyze", test_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    // Should either succeed or provide meaningful error about ODS support
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Should mention ODS in error message if not supported
        assert!(
            stderr.contains("ods") || stderr.contains("OpenDocument") || stderr.contains("format")
        );
    }
}
