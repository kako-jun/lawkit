// Temporary mock implementation of diffx-core functionality
// TODO: Remove this when diffx reboot is complete and diffx-core is available

use serde_json::Value;

/// Mock DiffResult enum matching diffx-core's structure
#[derive(Debug, Clone)]
pub enum DiffResult {
    /// Value was modified
    Modified(String, Value, Value),
    /// Value was added
    Added(String, Value),
    /// Value was removed
    Removed(String, Value),
    /// Type was changed
    TypeChanged(String, Value, Value),
    /// No difference
    Unchanged,
}

/// Mock diff function - returns empty result for now
pub fn diff(_old: &Value, _new: &Value, _options: Option<()>) -> Result<Vec<DiffResult>, String> {
    // Return empty diff results - indicates no differences detected
    // This is a safe fallback until diffx-core is available
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_mock_diff() {
        let old = json!({"a": 1});
        let new = json!({"a": 2});
        let result = diff(&old, &new, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0); // Mock returns empty diff
    }
}
