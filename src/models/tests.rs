//! Test-related data models from the Develocity API.

use serde::{Deserialize, Serialize};

/// Test results from `/api/builds/{id}/gradle-tests`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleTests {
    /// List of test executions.
    #[serde(default)]
    pub tests: Vec<GradleTestExecution>,
    /// Summary statistics.
    pub summary: Option<TestSummary>,
}

/// Individual test execution result.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleTestExecution {
    /// The task path that executed the test (e.g., ":app:test").
    pub task_path: String,
    /// The test suite/class name.
    pub suite: String,
    /// The test method name. None for suite-level results.
    pub name: Option<String>,
    /// Fully qualified class name.
    pub class_name: Option<String>,
    /// Test outcome: passed, failed, skipped.
    pub outcome: TestOutcome,
    /// Test duration in milliseconds.
    pub duration: i64,
    /// Failure message if the test failed.
    pub failure_message: Option<String>,
    /// Stacktrace if the test failed.
    pub stacktrace: Option<String>,
    /// Test output (stdout/stderr) if captured.
    pub output: Option<TestOutput>,
}

/// Test outcome enumeration.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TestOutcome {
    Passed,
    Failed,
    Skipped,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for TestOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestOutcome::Passed => write!(f, "passed"),
            TestOutcome::Failed => write!(f, "failed"),
            TestOutcome::Skipped => write!(f, "skipped"),
            TestOutcome::Unknown => write!(f, "unknown"),
        }
    }
}

/// Captured test output.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestOutput {
    /// Standard output from the test.
    pub stdout: Option<String>,
    /// Standard error from the test.
    pub stderr: Option<String>,
}

/// Summary statistics for test execution.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestSummary {
    /// Total number of tests.
    pub total: i32,
    /// Number of passed tests.
    pub passed: i32,
    /// Number of failed tests.
    pub failed: i32,
    /// Number of skipped tests.
    pub skipped: i32,
    /// Total duration in milliseconds.
    pub duration: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== TestOutcome Tests ====================

    #[test]
    fn test_outcome_deserialize_passed() {
        let json = r#""passed""#;
        let outcome: TestOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, TestOutcome::Passed);
    }

    #[test]
    fn test_outcome_deserialize_failed() {
        let json = r#""failed""#;
        let outcome: TestOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, TestOutcome::Failed);
    }

    #[test]
    fn test_outcome_deserialize_skipped() {
        let json = r#""skipped""#;
        let outcome: TestOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, TestOutcome::Skipped);
    }

    #[test]
    fn test_outcome_deserialize_unknown_value() {
        // Unknown values should deserialize to Unknown variant
        let json = r#""flaky""#;
        let outcome: TestOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, TestOutcome::Unknown);
    }

    #[test]
    fn test_outcome_deserialize_unexpected_value() {
        // Any unexpected string should deserialize to Unknown
        let json = r#""some_future_status""#;
        let outcome: TestOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, TestOutcome::Unknown);
    }

    #[test]
    fn test_outcome_display_passed() {
        assert_eq!(TestOutcome::Passed.to_string(), "passed");
    }

    #[test]
    fn test_outcome_display_failed() {
        assert_eq!(TestOutcome::Failed.to_string(), "failed");
    }

    #[test]
    fn test_outcome_display_skipped() {
        assert_eq!(TestOutcome::Skipped.to_string(), "skipped");
    }

    #[test]
    fn test_outcome_display_unknown() {
        assert_eq!(TestOutcome::Unknown.to_string(), "unknown");
    }

    #[test]
    fn test_outcome_serialize_roundtrip() {
        for outcome in [
            TestOutcome::Passed,
            TestOutcome::Failed,
            TestOutcome::Skipped,
        ] {
            let json = serde_json::to_string(&outcome).unwrap();
            let deserialized: TestOutcome = serde_json::from_str(&json).unwrap();
            assert_eq!(outcome, deserialized);
        }
    }

    // ==================== GradleTests Tests ====================

    #[test]
    fn test_gradle_tests_deserialize_empty_tests() {
        let json = r#"{"tests": [], "summary": null}"#;
        let tests: GradleTests = serde_json::from_str(json).unwrap();
        assert!(tests.tests.is_empty());
        assert!(tests.summary.is_none());
    }

    #[test]
    fn test_gradle_tests_deserialize_missing_tests_field() {
        // tests field has #[serde(default)] so missing field should default to empty vec
        let json = r#"{"summary": null}"#;
        let tests: GradleTests = serde_json::from_str(json).unwrap();
        assert!(tests.tests.is_empty());
        assert!(tests.summary.is_none());
    }

    #[test]
    fn test_gradle_tests_deserialize_with_summary() {
        let json = r#"{
            "tests": [],
            "summary": {
                "total": 100,
                "passed": 95,
                "failed": 3,
                "skipped": 2,
                "duration": 45000
            }
        }"#;
        let tests: GradleTests = serde_json::from_str(json).unwrap();
        assert!(tests.tests.is_empty());

        let summary = tests.summary.unwrap();
        assert_eq!(summary.total, 100);
        assert_eq!(summary.passed, 95);
        assert_eq!(summary.failed, 3);
        assert_eq!(summary.skipped, 2);
        assert_eq!(summary.duration, 45000);
    }

    #[test]
    fn test_gradle_tests_deserialize_full_test_data() {
        let json = r#"{
            "tests": [
                {
                    "taskPath": ":app:test",
                    "suite": "com.example.UserServiceTest",
                    "name": "testCreateUser",
                    "className": "com.example.UserServiceTest",
                    "outcome": "passed",
                    "duration": 150,
                    "failureMessage": null,
                    "stacktrace": null,
                    "output": null
                },
                {
                    "taskPath": ":app:test",
                    "suite": "com.example.UserServiceTest",
                    "name": "testDeleteUser",
                    "className": "com.example.UserServiceTest",
                    "outcome": "failed",
                    "duration": 200,
                    "failureMessage": "Expected true but was false",
                    "stacktrace": "at com.example.UserServiceTest.testDeleteUser(UserServiceTest.java:42)",
                    "output": {
                        "stdout": "Starting test...",
                        "stderr": "Error occurred"
                    }
                }
            ],
            "summary": {
                "total": 2,
                "passed": 1,
                "failed": 1,
                "skipped": 0,
                "duration": 350
            }
        }"#;

        let gradle_tests: GradleTests = serde_json::from_str(json).unwrap();

        // Check tests array
        assert_eq!(gradle_tests.tests.len(), 2);

        // Check first test (passed)
        let test1 = &gradle_tests.tests[0];
        assert_eq!(test1.task_path, ":app:test");
        assert_eq!(test1.suite, "com.example.UserServiceTest");
        assert_eq!(test1.name, Some("testCreateUser".to_string()));
        assert_eq!(
            test1.class_name,
            Some("com.example.UserServiceTest".to_string())
        );
        assert_eq!(test1.outcome, TestOutcome::Passed);
        assert_eq!(test1.duration, 150);
        assert!(test1.failure_message.is_none());
        assert!(test1.stacktrace.is_none());
        assert!(test1.output.is_none());

        // Check second test (failed)
        let test2 = &gradle_tests.tests[1];
        assert_eq!(test2.task_path, ":app:test");
        assert_eq!(test2.outcome, TestOutcome::Failed);
        assert_eq!(test2.duration, 200);
        assert_eq!(
            test2.failure_message,
            Some("Expected true but was false".to_string())
        );
        assert!(test2.stacktrace.is_some());

        let output = test2.output.as_ref().unwrap();
        assert_eq!(output.stdout, Some("Starting test...".to_string()));
        assert_eq!(output.stderr, Some("Error occurred".to_string()));

        // Check summary
        let summary = gradle_tests.summary.unwrap();
        assert_eq!(summary.total, 2);
        assert_eq!(summary.passed, 1);
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.skipped, 0);
        assert_eq!(summary.duration, 350);
    }

    // ==================== GradleTestExecution Tests ====================

    #[test]
    fn test_execution_deserialize_minimal() {
        // Test with only required fields
        let json = r#"{
            "taskPath": ":test",
            "suite": "TestSuite",
            "outcome": "passed",
            "duration": 100
        }"#;

        let execution: GradleTestExecution = serde_json::from_str(json).unwrap();
        assert_eq!(execution.task_path, ":test");
        assert_eq!(execution.suite, "TestSuite");
        assert!(execution.name.is_none());
        assert!(execution.class_name.is_none());
        assert_eq!(execution.outcome, TestOutcome::Passed);
        assert_eq!(execution.duration, 100);
        assert!(execution.failure_message.is_none());
        assert!(execution.stacktrace.is_none());
        assert!(execution.output.is_none());
    }

    #[test]
    fn test_execution_deserialize_skipped() {
        let json = r#"{
            "taskPath": ":test",
            "suite": "TestSuite",
            "name": "skippedTest",
            "outcome": "skipped",
            "duration": 0
        }"#;

        let execution: GradleTestExecution = serde_json::from_str(json).unwrap();
        assert_eq!(execution.outcome, TestOutcome::Skipped);
        assert_eq!(execution.duration, 0);
    }

    // ==================== TestOutput Tests ====================

    #[test]
    fn test_output_deserialize_both_streams() {
        let json = r#"{
            "stdout": "Standard output content",
            "stderr": "Standard error content"
        }"#;

        let output: TestOutput = serde_json::from_str(json).unwrap();
        assert_eq!(output.stdout, Some("Standard output content".to_string()));
        assert_eq!(output.stderr, Some("Standard error content".to_string()));
    }

    #[test]
    fn test_output_deserialize_only_stdout() {
        let json = r#"{
            "stdout": "Only stdout",
            "stderr": null
        }"#;

        let output: TestOutput = serde_json::from_str(json).unwrap();
        assert_eq!(output.stdout, Some("Only stdout".to_string()));
        assert!(output.stderr.is_none());
    }

    #[test]
    fn test_output_deserialize_empty() {
        let json = r#"{
            "stdout": null,
            "stderr": null
        }"#;

        let output: TestOutput = serde_json::from_str(json).unwrap();
        assert!(output.stdout.is_none());
        assert!(output.stderr.is_none());
    }

    // ==================== TestSummary Tests ====================

    #[test]
    fn test_summary_deserialize() {
        let json = r#"{
            "total": 500,
            "passed": 480,
            "failed": 15,
            "skipped": 5,
            "duration": 120000
        }"#;

        let summary: TestSummary = serde_json::from_str(json).unwrap();
        assert_eq!(summary.total, 500);
        assert_eq!(summary.passed, 480);
        assert_eq!(summary.failed, 15);
        assert_eq!(summary.skipped, 5);
        assert_eq!(summary.duration, 120000);
    }

    #[test]
    fn test_summary_serialize_roundtrip() {
        let summary = TestSummary {
            total: 10,
            passed: 8,
            failed: 1,
            skipped: 1,
            duration: 5000,
        };

        let json = serde_json::to_string(&summary).unwrap();
        let deserialized: TestSummary = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.total, summary.total);
        assert_eq!(deserialized.passed, summary.passed);
        assert_eq!(deserialized.failed, summary.failed);
        assert_eq!(deserialized.skipped, summary.skipped);
        assert_eq!(deserialized.duration, summary.duration);
    }

    #[test]
    fn test_summary_all_zeros() {
        let json = r#"{
            "total": 0,
            "passed": 0,
            "failed": 0,
            "skipped": 0,
            "duration": 0
        }"#;

        let summary: TestSummary = serde_json::from_str(json).unwrap();
        assert_eq!(summary.total, 0);
        assert_eq!(summary.passed, 0);
        assert_eq!(summary.failed, 0);
        assert_eq!(summary.skipped, 0);
        assert_eq!(summary.duration, 0);
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_gradle_tests_with_mixed_outcomes() {
        let json = r#"{
            "tests": [
                {"taskPath": ":test", "suite": "A", "name": "t1", "outcome": "passed", "duration": 10},
                {"taskPath": ":test", "suite": "A", "name": "t2", "outcome": "failed", "duration": 20},
                {"taskPath": ":test", "suite": "A", "name": "t3", "outcome": "skipped", "duration": 0},
                {"taskPath": ":test", "suite": "A", "name": "t4", "outcome": "unknown_status", "duration": 5}
            ]
        }"#;

        let tests: GradleTests = serde_json::from_str(json).unwrap();
        assert_eq!(tests.tests.len(), 4);
        assert_eq!(tests.tests[0].outcome, TestOutcome::Passed);
        assert_eq!(tests.tests[1].outcome, TestOutcome::Failed);
        assert_eq!(tests.tests[2].outcome, TestOutcome::Skipped);
        assert_eq!(tests.tests[3].outcome, TestOutcome::Unknown);
    }

    #[test]
    fn test_execution_with_multiline_failure_message() {
        let json = r#"{
            "taskPath": ":test",
            "suite": "TestSuite",
            "name": "failingTest",
            "outcome": "failed",
            "duration": 100,
            "failureMessage": "Assertion failed:\nExpected: 42\n  Actual: 0"
        }"#;

        let execution: GradleTestExecution = serde_json::from_str(json).unwrap();
        assert!(execution.failure_message.as_ref().unwrap().contains('\n'));
        assert!(execution
            .failure_message
            .as_ref()
            .unwrap()
            .contains("Expected: 42"));
    }

    #[test]
    fn test_execution_with_long_stacktrace() {
        let stacktrace = "at line1\\nat line2\\nat line3\\nat line4\\nat line5";
        let json = format!(
            r#"{{
                "taskPath": ":test",
                "suite": "TestSuite",
                "outcome": "failed",
                "duration": 100,
                "stacktrace": "{}"
            }}"#,
            stacktrace
        );

        let execution: GradleTestExecution = serde_json::from_str(&json).unwrap();
        // The actual stacktrace will have newlines after JSON parsing
        assert!(execution.stacktrace.is_some());
        assert!(execution.stacktrace.as_ref().unwrap().contains("line1"));
        assert!(execution.stacktrace.as_ref().unwrap().contains("line5"));
    }
}
