//! Output formatting for the CLI.

pub mod human;
pub mod json;

use crate::client::GradleBuildDetails;
use crate::models::{
    GradleAttributes, GradleBuildCachePerformance, GradleDeprecationEntry, GradleFailures,
    GradleTests,
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
    /// Task execution / build cache performance data (if requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution: Option<TaskExecutionOutput>,
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
    /// Number of flaky tests.
    pub flaky: i32,
    /// Number of not-selected tests.
    pub not_selected: i32,
    /// Total duration in milliseconds.
    pub duration_ms: i64,
    /// Pass rate as a percentage.
    pub pass_rate: f64,
}

/// Individual test execution output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestExecutionOutput {
    /// The work unit that executed the test (e.g., ":test", ":app:test").
    pub work_unit: String,
    /// The fully qualified test container name (e.g., "org.example.MyTest").
    pub class_name: String,
    /// The test method name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_name: Option<String>,
    /// Test outcome: passed, failed, skipped, flaky, not selected.
    pub outcome: String,
    /// Test duration in milliseconds.
    pub duration_ms: i64,
    /// Number of executions (>1 indicates retries).
    pub execution_count: usize,
}

/// Task execution / build cache performance output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskExecutionOutput {
    /// Summary of build timing.
    pub summary: TaskExecutionSummaryOutput,
    /// Avoidance savings summary.
    pub avoidance_savings: AvoidanceSavingsOutput,
    /// Per-task execution entries.
    pub tasks: Vec<TaskEntryOutput>,
}

/// Summary of build cache performance timing.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskExecutionSummaryOutput {
    /// Total build time in milliseconds.
    pub build_time_ms: i64,
    /// Effective task execution time in milliseconds.
    pub effective_task_execution_time_ms: i64,
    /// Serial task execution time in milliseconds.
    pub serial_task_execution_time_ms: i64,
    /// Serialization factor.
    pub serialization_factor: f64,
    /// Total number of tasks.
    pub total_tasks: usize,
    /// Number of avoided tasks (up-to-date + cached).
    pub avoided_tasks: usize,
    /// Number of executed tasks.
    pub executed_tasks: usize,
}

/// Avoidance savings output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvoidanceSavingsOutput {
    /// Total avoidance savings in milliseconds.
    pub total_ms: i64,
    /// Avoidance savings ratio (0.0 to 1.0).
    pub ratio: f64,
    /// Savings from up-to-date avoidance in milliseconds.
    pub up_to_date_ms: i64,
    /// Savings from local build cache in milliseconds.
    pub local_build_cache_ms: i64,
    /// Savings from remote build cache in milliseconds.
    pub remote_build_cache_ms: i64,
}

/// Per-task execution entry output.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskEntryOutput {
    /// Gradle task path.
    pub task_path: String,
    /// Avoidance outcome.
    pub outcome: String,
    /// Task duration in milliseconds.
    pub duration_ms: i64,
    /// Whether this task failed.
    pub has_failed: bool,
    /// Fully qualified task type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// Cache key hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key: Option<String>,
    /// Cache artifact size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_artifact_size: Option<i64>,
    /// Non-cacheability reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_cacheability_reason: Option<String>,
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

        let task_execution = details
            .build_cache_performance
            .map(TaskExecutionOutput::from_performance);

        Self {
            build_id: details.build.id,
            build_scan_url,
            result,
            deprecations,
            failures,
            tests,
            task_execution,
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
    fn from_tests(tests: GradleTests, _verbose: bool) -> Self {
        let dist = &tests.summary.test_cases_outcome_distribution;
        let total = dist.total;
        let passed = dist.passed;

        let summary = TestSummaryOutput {
            total: dist.total,
            passed: dist.passed,
            failed: dist.failed,
            skipped: dist.skipped,
            flaky: dist.flaky,
            not_selected: dist.not_selected,
            duration_ms: tests.summary.duration.total,
            pass_rate: if total > 0 {
                (passed as f64 / total as f64) * 100.0
            } else {
                0.0
            },
        };

        let test_executions = tests
            .flatten_test_cases()
            .into_iter()
            .map(|tc| TestExecutionOutput {
                work_unit: tc.work_unit,
                class_name: tc.container_name,
                test_name: Some(tc.test_name),
                outcome: tc.outcome.to_string(),
                duration_ms: tc.duration_ms,
                execution_count: tc.execution_count,
            })
            .collect();

        Self {
            summary,
            tests: test_executions,
        }
    }
}

impl TaskExecutionOutput {
    fn from_performance(perf: GradleBuildCachePerformance) -> Self {
        let total_tasks = perf.task_execution.len();
        let avoided_tasks = perf
            .task_execution
            .iter()
            .filter(|t| {
                matches!(
                    t.avoidance_outcome,
                    crate::models::AvoidanceOutcome::AvoidedUpToDate
                        | crate::models::AvoidanceOutcome::AvoidedFromLocalCache
                        | crate::models::AvoidanceOutcome::AvoidedFromRemoteCache
                        | crate::models::AvoidanceOutcome::AvoidedUnknownReason
                )
            })
            .count();
        let executed_tasks = perf
            .task_execution
            .iter()
            .filter(|t| {
                matches!(
                    t.avoidance_outcome,
                    crate::models::AvoidanceOutcome::ExecutedCacheable
                        | crate::models::AvoidanceOutcome::ExecutedNotCacheable
                        | crate::models::AvoidanceOutcome::ExecutedUnknownCacheability
                )
            })
            .count();

        let summary = TaskExecutionSummaryOutput {
            build_time_ms: perf.build_time,
            effective_task_execution_time_ms: perf.effective_task_execution_time,
            serial_task_execution_time_ms: perf.serial_task_execution_time,
            serialization_factor: perf.serialization_factor,
            total_tasks,
            avoided_tasks,
            executed_tasks,
        };

        let savings = &perf.task_avoidance_savings_summary;
        let avoidance_savings = AvoidanceSavingsOutput {
            total_ms: savings.total,
            ratio: savings.ratio,
            up_to_date_ms: savings.up_to_date,
            local_build_cache_ms: savings.local_build_cache,
            remote_build_cache_ms: savings.remote_build_cache,
        };

        let tasks = perf
            .task_execution
            .into_iter()
            .map(|entry| {
                let non_cacheability_reason = entry.non_cacheability_category.as_ref().map(|cat| {
                    match &entry.non_cacheability_reason {
                        Some(reason) => format!("{}: {}", cat, reason),
                        None => cat.to_string(),
                    }
                });

                TaskEntryOutput {
                    task_path: entry.task_path,
                    outcome: entry.avoidance_outcome.to_string(),
                    duration_ms: entry.duration,
                    has_failed: entry.has_failed,
                    task_type: Some(entry.task_type),
                    cache_key: entry.cache_key,
                    cache_artifact_size: entry.cache_artifact_size,
                    non_cacheability_reason,
                }
            })
            .collect();

        Self {
            summary,
            avoidance_savings,
            tasks,
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
    use crate::models::{
        BuildTestOrContainer, BuildTestOrContainerDuration, BuildTestOrContainerExecution,
        BuildTestOrContainerOutcome, BuildTestWorkUnit, BuildTestsDuration, BuildTestsResponse,
        BuildTestsSummary, TestOutcome, TestOutcomeDistribution, TestWorkUnitOutcome,
    };

    // ==================== Helper Functions ====================

    fn make_summary(
        total: i32,
        passed: i32,
        failed: i32,
        skipped: i32,
        flaky: i32,
        not_selected: i32,
        duration_ms: i64,
    ) -> BuildTestsSummary {
        BuildTestsSummary {
            duration: BuildTestsDuration {
                total: duration_ms,
                serial: Some(duration_ms),
            },
            test_cases_outcome_distribution: TestOutcomeDistribution {
                total,
                passed,
                failed,
                skipped,
                flaky,
                not_selected,
            },
            test_containers_outcome_distribution: TestOutcomeDistribution {
                total: 1,
                passed: 1,
                failed: 0,
                skipped: 0,
                flaky: 0,
                not_selected: 0,
            },
        }
    }

    fn make_leaf_test(name: &str, outcome: TestOutcome, duration_ms: i64) -> BuildTestOrContainer {
        BuildTestOrContainer {
            name: name.to_string(),
            duration: BuildTestOrContainerDuration {
                total: Some(duration_ms),
                own: None,
                serial: None,
            },
            outcome: BuildTestOrContainerOutcome {
                overall: outcome.clone(),
                own: None,
                children: None,
            },
            executions: vec![BuildTestOrContainerExecution {
                duration: BuildTestOrContainerDuration {
                    total: Some(duration_ms),
                    own: None,
                    serial: None,
                },
                outcome: BuildTestOrContainerOutcome {
                    overall: outcome,
                    own: None,
                    children: None,
                },
            }],
            children: vec![],
        }
    }

    fn make_container(name: &str, children: Vec<BuildTestOrContainer>) -> BuildTestOrContainer {
        BuildTestOrContainer {
            name: name.to_string(),
            duration: BuildTestOrContainerDuration {
                total: Some(100),
                own: Some(5),
                serial: Some(100),
            },
            outcome: BuildTestOrContainerOutcome {
                overall: TestOutcome::Passed,
                own: Some(TestOutcome::Passed),
                children: Some(TestOutcome::Passed),
            },
            executions: vec![],
            children,
        }
    }

    fn make_work_unit(name: &str, tests: Vec<BuildTestOrContainer>) -> BuildTestWorkUnit {
        BuildTestWorkUnit {
            name: name.to_string(),
            duration: BuildTestOrContainerDuration {
                total: Some(200),
                own: Some(10),
                serial: Some(200),
            },
            outcome: TestWorkUnitOutcome::Passed,
            tests,
        }
    }

    fn make_gradle_tests(
        summary: BuildTestsSummary,
        work_units: Vec<BuildTestWorkUnit>,
    ) -> GradleTests {
        BuildTestsResponse {
            summary,
            work_units,
        }
    }

    // ==================== TestsOutput::from_tests() Tests ====================

    #[test]
    fn test_tests_output_from_tests_uses_api_summary() {
        let summary = make_summary(10, 8, 1, 1, 0, 0, 5000);
        let gradle_tests = make_gradle_tests(summary, vec![]);

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.summary.total, 10);
        assert_eq!(output.summary.passed, 8);
        assert_eq!(output.summary.failed, 1);
        assert_eq!(output.summary.skipped, 1);
        assert_eq!(output.summary.flaky, 0);
        assert_eq!(output.summary.not_selected, 0);
        assert_eq!(output.summary.duration_ms, 5000);
        assert!((output.summary.pass_rate - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_tests_output_pass_rate_calculation_zero_total() {
        let summary = make_summary(0, 0, 0, 0, 0, 0, 0);
        let gradle_tests = make_gradle_tests(summary, vec![]);

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.summary.pass_rate, 0.0);
    }

    #[test]
    fn test_tests_output_pass_rate_100_percent() {
        let summary = make_summary(2, 2, 0, 0, 0, 0, 200);
        let gradle_tests = make_gradle_tests(summary, vec![]);

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert!((output.summary.pass_rate - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_tests_output_pass_rate_0_percent() {
        let summary = make_summary(2, 0, 2, 0, 0, 0, 200);
        let gradle_tests = make_gradle_tests(summary, vec![]);

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.summary.pass_rate, 0.0);
    }

    #[test]
    fn test_tests_output_includes_flaky_and_not_selected() {
        let summary = make_summary(10, 5, 1, 1, 2, 1, 1000);
        let gradle_tests = make_gradle_tests(summary, vec![]);

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.summary.flaky, 2);
        assert_eq!(output.summary.not_selected, 1);
    }

    // ==================== TestExecutionOutput Tests ====================

    #[test]
    fn test_execution_output_from_flattened_tests() {
        let container = make_container(
            "com.example.TestSuite",
            vec![
                make_leaf_test("testA", TestOutcome::Passed, 100),
                make_leaf_test("testB", TestOutcome::Failed, 200),
            ],
        );
        let wu = make_work_unit(":app:test", vec![container]);
        let summary = make_summary(2, 1, 1, 0, 0, 0, 300);
        let gradle_tests = make_gradle_tests(summary, vec![wu]);

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.tests.len(), 2);

        assert_eq!(output.tests[0].work_unit, ":app:test");
        assert_eq!(output.tests[0].class_name, "com.example.TestSuite");
        assert_eq!(output.tests[0].test_name, Some("testA".to_string()));
        assert_eq!(output.tests[0].outcome, "passed");
        assert_eq!(output.tests[0].duration_ms, 100);
        assert_eq!(output.tests[0].execution_count, 1);

        assert_eq!(output.tests[1].work_unit, ":app:test");
        assert_eq!(output.tests[1].test_name, Some("testB".to_string()));
        assert_eq!(output.tests[1].outcome, "failed");
        assert_eq!(output.tests[1].duration_ms, 200);
    }

    #[test]
    fn test_execution_output_outcome_strings() {
        for (outcome, expected) in [
            (TestOutcome::Passed, "passed"),
            (TestOutcome::Failed, "failed"),
            (TestOutcome::Skipped, "skipped"),
            (TestOutcome::Flaky, "flaky"),
            (TestOutcome::NotSelected, "notSelected"),
        ] {
            let container = make_container("Suite", vec![make_leaf_test("test", outcome, 0)]);
            let wu = make_work_unit(":test", vec![container]);
            let summary = make_summary(1, 0, 0, 0, 0, 0, 0);
            let gradle_tests = make_gradle_tests(summary, vec![wu]);

            let output = TestsOutput::from_tests(gradle_tests, false);
            assert_eq!(output.tests[0].outcome, expected);
        }
    }

    #[test]
    fn test_execution_output_multiple_work_units() {
        let container1 = make_container(
            "AppTest",
            vec![make_leaf_test("test1", TestOutcome::Passed, 100)],
        );
        let container2 = make_container(
            "LibTest",
            vec![make_leaf_test("test2", TestOutcome::Passed, 200)],
        );
        let wu1 = make_work_unit(":app:test", vec![container1]);
        let wu2 = make_work_unit(":lib:test", vec![container2]);
        let summary = make_summary(2, 2, 0, 0, 0, 0, 300);
        let gradle_tests = make_gradle_tests(summary, vec![wu1, wu2]);

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.tests.len(), 2);
        assert_eq!(output.tests[0].work_unit, ":app:test");
        assert_eq!(output.tests[1].work_unit, ":lib:test");
    }

    #[test]
    fn test_execution_output_empty_work_units() {
        let summary = make_summary(0, 0, 0, 0, 0, 0, 0);
        let gradle_tests = make_gradle_tests(summary, vec![]);

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert!(output.tests.is_empty());
        assert_eq!(output.summary.total, 0);
    }

    #[test]
    fn test_execution_output_retries_counted() {
        // A test with 2 executions (retried once)
        let test = BuildTestOrContainer {
            name: "flakyTest".to_string(),
            duration: BuildTestOrContainerDuration {
                total: Some(40),
                own: None,
                serial: None,
            },
            outcome: BuildTestOrContainerOutcome {
                overall: TestOutcome::Flaky,
                own: None,
                children: None,
            },
            executions: vec![
                BuildTestOrContainerExecution {
                    duration: BuildTestOrContainerDuration {
                        total: Some(20),
                        own: None,
                        serial: None,
                    },
                    outcome: BuildTestOrContainerOutcome {
                        overall: TestOutcome::Failed,
                        own: None,
                        children: None,
                    },
                },
                BuildTestOrContainerExecution {
                    duration: BuildTestOrContainerDuration {
                        total: Some(20),
                        own: None,
                        serial: None,
                    },
                    outcome: BuildTestOrContainerOutcome {
                        overall: TestOutcome::Passed,
                        own: None,
                        children: None,
                    },
                },
            ],
            children: vec![],
        };

        let container = make_container("Suite", vec![test]);
        let wu = make_work_unit(":test", vec![container]);
        let summary = make_summary(1, 0, 0, 0, 1, 0, 40);
        let gradle_tests = make_gradle_tests(summary, vec![wu]);

        let output = TestsOutput::from_tests(gradle_tests, false);

        assert_eq!(output.tests.len(), 1);
        assert_eq!(output.tests[0].outcome, "flaky");
        assert_eq!(output.tests[0].execution_count, 2);
    }

    // ==================== TestSummaryOutput Serialization Tests ====================

    #[test]
    fn test_summary_output_json_serialization() {
        let summary = make_summary(100, 95, 3, 2, 0, 0, 60000);
        let gradle_tests = make_gradle_tests(summary, vec![]);

        let output = TestsOutput::from_tests(gradle_tests, false);
        let json = serde_json::to_string(&output.summary).unwrap();

        // Verify camelCase serialization
        assert!(json.contains("\"total\":100"));
        assert!(json.contains("\"passed\":95"));
        assert!(json.contains("\"failed\":3"));
        assert!(json.contains("\"skipped\":2"));
        assert!(json.contains("\"flaky\":0"));
        assert!(json.contains("\"notSelected\":0"));
        assert!(json.contains("\"durationMs\":60000"));
        assert!(json.contains("\"passRate\":95"));
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
}
