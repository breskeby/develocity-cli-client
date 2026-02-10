//! Output formatting for the CLI.

pub mod human;
pub mod json;

use crate::client::GradleBuildDetails;
use crate::models::{
    GradleAttributes, GradleDeprecationEntry, GradleFailures, GradleTests, TestOutcome,
};
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
    /// Test execution results (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests: Option<TestsOutput>,
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

/// Tests output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestsOutput {
    /// Summary statistics.
    pub summary: TestSummaryOutput,
    /// Individual test results.
    pub tests: Vec<TestExecutionOutput>,
}

/// Summary statistics for test execution.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestSummaryOutput {
    /// Total number of tests.
    pub total: i32,
    /// Number of passed tests.
    pub passed: i32,
    /// Number of failed tests.
    pub failed: i32,
    /// Number of skipped tests.
    pub skipped: i32,
    /// Total duration in milliseconds.
    pub duration_ms: i64,
    /// Pass rate as a percentage.
    pub pass_rate: f64,
}

/// Individual test execution output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestExecutionOutput {
    /// The task path that executed the test.
    pub task_path: String,
    /// The test class name.
    pub class_name: String,
    /// The test method name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_name: Option<String>,
    /// Test outcome: passed, failed, skipped.
    pub outcome: String,
    /// Test duration in milliseconds.
    pub duration_ms: i64,
    /// Failure message if the test failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// Stacktrace if the test failed (only included with verbose).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacktrace: Option<String>,
    /// Standard output from the test.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    /// Standard error from the test.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
}

impl BuildOutput {
    /// Create a BuildOutput from GradleBuildDetails.
    pub fn from_details(
        details: GradleBuildDetails,
        build_scan_url: String,
        verbose: bool,
    ) -> Self {
        let result = details.attributes.map(ResultOutput::from);

        let deprecations = details.deprecations.and_then(|deps| {
            deps.deprecations
                .map(|entries| entries.into_iter().map(DeprecationOutput::from).collect())
        });

        let failures = details
            .failures
            .map(|f| FailuresOutput::from_failures(f, verbose));

        let tests = details.tests.map(|t| TestsOutput::from_tests(t, verbose));

        Self {
            build_id: details.build.id,
            build_scan_url,
            result,
            deprecations,
            failures,
            tests,
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

impl TestsOutput {
    fn from_tests(tests: GradleTests, verbose: bool) -> Self {
        // Calculate summary if not provided
        let summary = tests
            .summary
            .map(|s| TestSummaryOutput {
                total: s.total,
                passed: s.passed,
                failed: s.failed,
                skipped: s.skipped,
                duration_ms: s.duration,
                pass_rate: if s.total > 0 {
                    (s.passed as f64 / s.total as f64) * 100.0
                } else {
                    0.0
                },
            })
            .unwrap_or_else(|| {
                // Calculate from individual tests if no summary provided
                let total = tests.tests.len() as i32;
                let passed = tests
                    .tests
                    .iter()
                    .filter(|t| t.outcome == TestOutcome::Passed)
                    .count() as i32;
                let failed = tests
                    .tests
                    .iter()
                    .filter(|t| t.outcome == TestOutcome::Failed)
                    .count() as i32;
                let skipped = tests
                    .tests
                    .iter()
                    .filter(|t| t.outcome == TestOutcome::Skipped)
                    .count() as i32;
                let duration_ms: i64 = tests.tests.iter().map(|t| t.duration).sum();

                TestSummaryOutput {
                    total,
                    passed,
                    failed,
                    skipped,
                    duration_ms,
                    pass_rate: if total > 0 {
                        (passed as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    },
                }
            });

        let test_executions = tests
            .tests
            .into_iter()
            .map(|t| {
                let (stdout, stderr) = if verbose {
                    t.output
                        .map(|o| (o.stdout, o.stderr))
                        .unwrap_or((None, None))
                } else {
                    (None, None)
                };

                TestExecutionOutput {
                    task_path: t.task_path,
                    class_name: t.class_name.unwrap_or_else(|| t.suite.clone()),
                    test_name: t.name,
                    outcome: t.outcome.to_string(),
                    duration_ms: t.duration,
                    failure_message: t.failure_message,
                    stacktrace: if verbose { t.stacktrace } else { None },
                    stdout,
                    stderr,
                }
            })
            .collect();

        Self {
            summary,
            tests: test_executions,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{GradleTestExecution, TestOutput, TestSummary};

    // ==================== Helper Functions ====================

    fn make_test_execution(
        outcome: TestOutcome,
        duration: i64,
        name: Option<&str>,
    ) -> GradleTestExecution {
        let is_failed = outcome == TestOutcome::Failed;
        GradleTestExecution {
            task_path: ":app:test".to_string(),
            suite: "com.example.TestSuite".to_string(),
            name: name.map(|s| s.to_string()),
            class_name: Some("com.example.TestSuite".to_string()),
            outcome,
            duration,
            failure_message: if is_failed {
                Some("Test failed".to_string())
            } else {
                None
            },
            stacktrace: if is_failed {
                Some("at com.example.Test:42".to_string())
            } else {
                None
            },
            output: None,
        }
    }

    // ==================== TestsOutput::from_tests() Tests ====================

    #[test]
    fn test_tests_output_from_tests_with_provided_summary() {
        let gradle_tests = GradleTests {
            tests: vec![make_test_execution(TestOutcome::Passed, 100, Some("test1"))],
            summary: Some(TestSummary {
                total: 10,
                passed: 8,
                failed: 1,
                skipped: 1,
                duration: 5000,
            }),
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        // Should use provided summary, not calculate from tests
        assert_eq!(output.summary.total, 10);
        assert_eq!(output.summary.passed, 8);
        assert_eq!(output.summary.failed, 1);
        assert_eq!(output.summary.skipped, 1);
        assert_eq!(output.summary.duration_ms, 5000);
        assert!((output.summary.pass_rate - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_tests_output_from_tests_calculates_summary_when_none() {
        let gradle_tests = GradleTests {
            tests: vec![
                make_test_execution(TestOutcome::Passed, 100, Some("test1")),
                make_test_execution(TestOutcome::Passed, 150, Some("test2")),
                make_test_execution(TestOutcome::Failed, 200, Some("test3")),
                make_test_execution(TestOutcome::Skipped, 0, Some("test4")),
            ],
            summary: None,
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        // Should calculate summary from tests
        assert_eq!(output.summary.total, 4);
        assert_eq!(output.summary.passed, 2);
        assert_eq!(output.summary.failed, 1);
        assert_eq!(output.summary.skipped, 1);
        assert_eq!(output.summary.duration_ms, 450); // 100 + 150 + 200 + 0
        assert!((output.summary.pass_rate - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_tests_output_pass_rate_calculation_zero_total() {
        // Test division by zero handling
        let gradle_tests = GradleTests {
            tests: vec![],
            summary: Some(TestSummary {
                total: 0,
                passed: 0,
                failed: 0,
                skipped: 0,
                duration: 0,
            }),
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        // Should handle zero total gracefully (0.0 pass rate, not NaN or panic)
        assert_eq!(output.summary.pass_rate, 0.0);
    }

    #[test]
    fn test_tests_output_pass_rate_calculation_empty_tests_no_summary() {
        // When no tests and no summary, calculated summary should have 0 pass rate
        let gradle_tests = GradleTests {
            tests: vec![],
            summary: None,
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.summary.total, 0);
        assert_eq!(output.summary.pass_rate, 0.0);
    }

    #[test]
    fn test_tests_output_pass_rate_100_percent() {
        let gradle_tests = GradleTests {
            tests: vec![
                make_test_execution(TestOutcome::Passed, 100, Some("test1")),
                make_test_execution(TestOutcome::Passed, 100, Some("test2")),
            ],
            summary: None,
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.summary.total, 2);
        assert_eq!(output.summary.passed, 2);
        assert!((output.summary.pass_rate - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_tests_output_pass_rate_0_percent() {
        let gradle_tests = GradleTests {
            tests: vec![
                make_test_execution(TestOutcome::Failed, 100, Some("test1")),
                make_test_execution(TestOutcome::Failed, 100, Some("test2")),
            ],
            summary: None,
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.summary.total, 2);
        assert_eq!(output.summary.passed, 0);
        assert_eq!(output.summary.pass_rate, 0.0);
    }

    // ==================== TestExecutionOutput Tests ====================

    #[test]
    fn test_execution_output_uses_class_name_when_present() {
        let execution = GradleTestExecution {
            task_path: ":test".to_string(),
            suite: "SuiteName".to_string(),
            name: Some("testMethod".to_string()),
            class_name: Some("com.example.ClassName".to_string()),
            outcome: TestOutcome::Passed,
            duration: 100,
            failure_message: None,
            stacktrace: None,
            output: None,
        };

        let gradle_tests = GradleTests {
            tests: vec![execution],
            summary: None,
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        // class_name should be used, not suite
        assert_eq!(output.tests[0].class_name, "com.example.ClassName");
    }

    #[test]
    fn test_execution_output_falls_back_to_suite_when_no_class_name() {
        let execution = GradleTestExecution {
            task_path: ":test".to_string(),
            suite: "SuiteName".to_string(),
            name: Some("testMethod".to_string()),
            class_name: None,
            outcome: TestOutcome::Passed,
            duration: 100,
            failure_message: None,
            stacktrace: None,
            output: None,
        };

        let gradle_tests = GradleTests {
            tests: vec![execution],
            summary: None,
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        // Should fall back to suite name
        assert_eq!(output.tests[0].class_name, "SuiteName");
    }

    #[test]
    fn test_execution_output_outcome_string() {
        for (outcome, expected) in [
            (TestOutcome::Passed, "passed"),
            (TestOutcome::Failed, "failed"),
            (TestOutcome::Skipped, "skipped"),
            (TestOutcome::Unknown, "unknown"),
        ] {
            let execution = GradleTestExecution {
                task_path: ":test".to_string(),
                suite: "Suite".to_string(),
                name: None,
                class_name: None,
                outcome,
                duration: 0,
                failure_message: None,
                stacktrace: None,
                output: None,
            };

            let gradle_tests = GradleTests {
                tests: vec![execution],
                summary: None,
            };

            let output = TestsOutput::from_tests(gradle_tests, false);
            assert_eq!(output.tests[0].outcome, expected);
        }
    }

    // ==================== Verbose Mode Tests ====================

    #[test]
    fn test_execution_output_includes_stacktrace_when_verbose() {
        let execution = GradleTestExecution {
            task_path: ":test".to_string(),
            suite: "Suite".to_string(),
            name: Some("failingTest".to_string()),
            class_name: None,
            outcome: TestOutcome::Failed,
            duration: 100,
            failure_message: Some("Assertion failed".to_string()),
            stacktrace: Some("at Test.java:42\nat Test.java:10".to_string()),
            output: None,
        };

        let gradle_tests = GradleTests {
            tests: vec![execution],
            summary: None,
        };

        // With verbose=true
        let output_verbose = TestsOutput::from_tests(gradle_tests.clone(), true);
        assert!(output_verbose.tests[0].stacktrace.is_some());

        // With verbose=false
        let output_non_verbose = TestsOutput::from_tests(gradle_tests, false);
        assert!(output_non_verbose.tests[0].stacktrace.is_none());
    }

    #[test]
    fn test_execution_output_includes_stdout_stderr_when_verbose() {
        let execution = GradleTestExecution {
            task_path: ":test".to_string(),
            suite: "Suite".to_string(),
            name: Some("test".to_string()),
            class_name: None,
            outcome: TestOutcome::Passed,
            duration: 100,
            failure_message: None,
            stacktrace: None,
            output: Some(TestOutput {
                stdout: Some("Standard output".to_string()),
                stderr: Some("Standard error".to_string()),
            }),
        };

        let gradle_tests = GradleTests {
            tests: vec![execution],
            summary: None,
        };

        // With verbose=true
        let output_verbose = TestsOutput::from_tests(gradle_tests.clone(), true);
        assert_eq!(
            output_verbose.tests[0].stdout,
            Some("Standard output".to_string())
        );
        assert_eq!(
            output_verbose.tests[0].stderr,
            Some("Standard error".to_string())
        );

        // With verbose=false
        let output_non_verbose = TestsOutput::from_tests(gradle_tests, false);
        assert!(output_non_verbose.tests[0].stdout.is_none());
        assert!(output_non_verbose.tests[0].stderr.is_none());
    }

    #[test]
    fn test_execution_output_no_output_field() {
        let execution = GradleTestExecution {
            task_path: ":test".to_string(),
            suite: "Suite".to_string(),
            name: Some("test".to_string()),
            class_name: None,
            outcome: TestOutcome::Passed,
            duration: 100,
            failure_message: None,
            stacktrace: None,
            output: None,
        };

        let gradle_tests = GradleTests {
            tests: vec![execution],
            summary: None,
        };

        // Even with verbose, if output is None, stdout/stderr should be None
        let output = TestsOutput::from_tests(gradle_tests, true);
        assert!(output.tests[0].stdout.is_none());
        assert!(output.tests[0].stderr.is_none());
    }

    // ==================== format_duration Tests ====================

    #[test]
    fn test_format_duration_milliseconds() {
        assert_eq!(format_duration(500), "500ms");
        assert_eq!(format_duration(0), "0ms");
        assert_eq!(format_duration(999), "999ms");
    }

    #[test]
    fn test_format_duration_seconds() {
        assert_eq!(format_duration(1000), "1s");
        assert_eq!(format_duration(5000), "5s");
        assert_eq!(format_duration(59000), "59s");
    }

    #[test]
    fn test_format_duration_minutes() {
        assert_eq!(format_duration(60000), "1m 0s");
        assert_eq!(format_duration(90000), "1m 30s");
        assert_eq!(format_duration(3599000), "59m 59s");
    }

    #[test]
    fn test_format_duration_hours() {
        assert_eq!(format_duration(3600000), "1h 0m 0s");
        assert_eq!(format_duration(7200000), "2h 0m 0s");
        assert_eq!(format_duration(3661000), "1h 1m 1s");
    }

    // ==================== TestSummaryOutput Serialization Tests ====================

    #[test]
    fn test_summary_output_json_serialization() {
        let gradle_tests = GradleTests {
            tests: vec![],
            summary: Some(TestSummary {
                total: 100,
                passed: 95,
                failed: 3,
                skipped: 2,
                duration: 60000,
            }),
        };

        let output = TestsOutput::from_tests(gradle_tests, false);
        let json = serde_json::to_string(&output.summary).unwrap();

        // Verify camelCase serialization
        assert!(json.contains("\"total\":100"));
        assert!(json.contains("\"passed\":95"));
        assert!(json.contains("\"failed\":3"));
        assert!(json.contains("\"skipped\":2"));
        assert!(json.contains("\"durationMs\":60000"));
        assert!(json.contains("\"passRate\":95"));
    }

    // ==================== Integration-style Tests ====================

    #[test]
    fn test_tests_output_preserves_failure_message() {
        let execution = GradleTestExecution {
            task_path: ":test".to_string(),
            suite: "Suite".to_string(),
            name: Some("failingTest".to_string()),
            class_name: None,
            outcome: TestOutcome::Failed,
            duration: 100,
            failure_message: Some("Expected 42 but got 0".to_string()),
            stacktrace: None,
            output: None,
        };

        let gradle_tests = GradleTests {
            tests: vec![execution],
            summary: None,
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(
            output.tests[0].failure_message,
            Some("Expected 42 but got 0".to_string())
        );
    }

    #[test]
    fn test_tests_output_multiple_tasks() {
        let gradle_tests = GradleTests {
            tests: vec![
                GradleTestExecution {
                    task_path: ":app:test".to_string(),
                    suite: "AppTest".to_string(),
                    name: Some("test1".to_string()),
                    class_name: None,
                    outcome: TestOutcome::Passed,
                    duration: 100,
                    failure_message: None,
                    stacktrace: None,
                    output: None,
                },
                GradleTestExecution {
                    task_path: ":lib:test".to_string(),
                    suite: "LibTest".to_string(),
                    name: Some("test2".to_string()),
                    class_name: None,
                    outcome: TestOutcome::Passed,
                    duration: 200,
                    failure_message: None,
                    stacktrace: None,
                    output: None,
                },
            ],
            summary: None,
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.tests.len(), 2);
        assert_eq!(output.tests[0].task_path, ":app:test");
        assert_eq!(output.tests[1].task_path, ":lib:test");
    }

    #[test]
    fn test_tests_output_unknown_outcome_counted_separately() {
        let gradle_tests = GradleTests {
            tests: vec![
                make_test_execution(TestOutcome::Passed, 100, Some("test1")),
                GradleTestExecution {
                    task_path: ":test".to_string(),
                    suite: "Suite".to_string(),
                    name: Some("unknownTest".to_string()),
                    class_name: None,
                    outcome: TestOutcome::Unknown,
                    duration: 50,
                    failure_message: None,
                    stacktrace: None,
                    output: None,
                },
            ],
            summary: None,
        };

        let output = TestsOutput::from_tests(gradle_tests, false);

        // Unknown should not be counted as passed, failed, or skipped
        assert_eq!(output.summary.total, 2);
        assert_eq!(output.summary.passed, 1);
        assert_eq!(output.summary.failed, 0);
        assert_eq!(output.summary.skipped, 0);
        // Pass rate: 1 passed / 2 total = 50%
        assert!((output.summary.pass_rate - 50.0).abs() < 0.01);
    }
}
