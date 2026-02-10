//! JSON output formatter.

use crate::output::BuildOutput;

/// Format the build output as JSON.
pub fn format(output: &BuildOutput) -> String {
    serde_json::to_string_pretty(output)
        .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize output: {}\"}}", e))
}
