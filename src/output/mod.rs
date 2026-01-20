//! Output formatting for the CLI.

pub mod human;
pub mod json;

use crate::client::GradleBuildDetails;
use crate::models::{GradleAttributes, GradleDeprecationEntry, GradleFailures};
use chrono::{TimeZone, Utc};
use serde::Serialize;

/// Combined output model for all build information.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildOutput {
    /// The Build Scan ID.
    pub build_id: String,
    /// URL to view the build scan in the web UI.
    pub build_scan_url: String,
    /// Build result information (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ResultOutput>,
    /// Deprecation information (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecations: Option<Vec<DeprecationOutput>>,
    /// Failure information (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<FailuresOutput>,
}

/// Build result output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultOutput {
    /// Build outcome: "success" or "failed".
    pub outcome: String,
    /// Whether the build failed.
    pub has_failed: bool,
    /// Whether there were verification failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verification_failure: Option<bool>,
    /// Whether there were non-verification failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_non_verification_failure: Option<bool>,
    /// Root project name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    /// Gradle version.
    pub gradle_version: String,
    /// Build duration in milliseconds.
    pub build_duration_ms: i64,
    /// Build start time in ISO 8601 format.
    pub build_start_time: String,
    /// Requested tasks.
    pub requested_tasks: Vec<String>,
    /// Build tags.
    pub tags: Vec<String>,
    /// Username that ran the build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Hostname where the build ran.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

/// Deprecation output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeprecationOutput {
    /// Description of the deprecation.
    pub summary: String,
    /// When the feature will be removed.
    pub removal_details: String,
    /// Advice on how to fix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advice: Option<String>,
    /// Documentation URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    /// Number of usages.
    pub usage_count: usize,
    /// Usage details.
    pub usages: Vec<DeprecationUsageOutput>,
}

/// Deprecation usage output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeprecationUsageOutput {
    /// Owner type (plugin, script, task, unknown).
    #[serde(rename = "type")]
    pub owner_type: String,
    /// Location of the usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Contextual advice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextual_advice: Option<String>,
}

/// Failures output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FailuresOutput {
    /// Build failures.
    pub build_failures: Vec<BuildFailureOutput>,
    /// Test failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_failures: Option<Vec<TestFailureOutput>>,
}

/// Build failure output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildFailureOutput {
    /// Failure header.
    pub header: String,
    /// Failure message.
    pub message: String,
    /// Task path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_path: Option<String>,
    /// Location in code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Stacktrace (only included with verbose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacktrace: Option<String>,
}

/// Test failure output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestFailureOutput {
    /// Test class name.
    pub class_name: String,
    /// Test method name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_name: Option<String>,
    /// Failure message.
    pub message: String,
    /// Stacktrace (only included with verbose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacktrace: Option<String>,
}

impl BuildOutput {
    /// Create a BuildOutput from GradleBuildDetails.
    pub fn from_details(
        details: GradleBuildDetails,
        build_scan_url: String,
        verbose: bool,
    ) -> Self {
        let result = details.attributes.map(|attrs| ResultOutput::from(attrs));

        let deprecations = details.deprecations.and_then(|deps| {
            deps.deprecations.map(|entries| {
                entries
                    .into_iter()
                    .map(DeprecationOutput::from)
                    .collect()
            })
        });

        let failures = details
            .failures
            .map(|f| FailuresOutput::from_failures(f, verbose));

        Self {
            build_id: details.build.id,
            build_scan_url,
            result,
            deprecations,
            failures,
        }
    }
}

impl From<GradleAttributes> for ResultOutput {
    fn from(attrs: GradleAttributes) -> Self {
        let build_start_time = Utc
            .timestamp_millis_opt(attrs.build_start_time)
            .single()
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| attrs.build_start_time.to_string());

        let hostname = attrs
            .environment
            .public_hostname
            .or(attrs.environment.local_hostname);

        Self {
            outcome: if attrs.has_failed {
                "failed".to_string()
            } else {
                "success".to_string()
            },
            has_failed: attrs.has_failed,
            has_verification_failure: attrs.has_verification_failure,
            has_non_verification_failure: attrs.has_non_verification_failure,
            project_name: attrs.root_project_name,
            gradle_version: attrs.gradle_version,
            build_duration_ms: attrs.build_duration,
            build_start_time,
            requested_tasks: attrs.requested_tasks,
            tags: attrs.tags,
            username: attrs.environment.username,
            hostname,
        }
    }
}

impl From<GradleDeprecationEntry> for DeprecationOutput {
    fn from(entry: GradleDeprecationEntry) -> Self {
        let usage_count = entry.usages.len();
        let usages = entry
            .usages
            .into_iter()
            .map(|u| DeprecationUsageOutput {
                owner_type: u.owner.owner_type.to_string(),
                location: u.owner.location,
                contextual_advice: u.contextual_advice,
            })
            .collect();

        Self {
            summary: entry.summary,
            removal_details: entry.removal_details,
            advice: entry.advice,
            documentation_url: entry.documentation_url,
            usage_count,
            usages,
        }
    }
}

impl FailuresOutput {
    fn from_failures(failures: GradleFailures, verbose: bool) -> Self {
        let build_failures = failures
            .build_failures
            .into_iter()
            .map(|f| BuildFailureOutput {
                header: f.header,
                message: f.message,
                task_path: f.task_path,
                location: f.location,
                stacktrace: if verbose { f.stacktrace } else { None },
            })
            .collect();

        let test_failures = failures.test_failures.map(|tests| {
            tests
                .into_iter()
                .map(|t| TestFailureOutput {
                    class_name: t.id.suite_name,
                    test_name: t.id.test_name,
                    message: t.message,
                    stacktrace: if verbose { t.stacktrace } else { None },
                })
                .collect()
        });

        Self {
            build_failures,
            test_failures,
        }
    }
}

/// Format a duration in milliseconds to a human-readable string.
pub fn format_duration(millis: i64) -> String {
    let total_secs = millis / 1000;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else if seconds > 0 {
        format!("{}s", seconds)
    } else {
        format!("{}ms", millis)
    }
}

/// Format a Unix timestamp in milliseconds to a human-readable string.
pub fn format_timestamp(millis: i64) -> String {
    Utc.timestamp_millis_opt(millis)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| millis.to_string())
}
