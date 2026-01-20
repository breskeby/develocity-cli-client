//! Failure-related data models from the Develocity API.

use serde::{Deserialize, Serialize};

/// Build and test failures from `/api/builds/{id}/gradle-failures`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleFailures {
    /// List of build failures.
    #[serde(default)]
    pub build_failures: Vec<GradleBuildFailure>,
    /// List of test failures. May be null if tests are not captured.
    pub test_failures: Option<Vec<TestFailure>>,
}

/// A build failure.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleBuildFailure {
    /// Failure header (e.g., "Task :compileJava FAILED").
    pub header: String,
    /// Failure message.
    pub message: String,
    /// Relevant console log output.
    pub relevant_log: Option<String>,
    /// Failure location (build script or source file).
    pub location: Option<String>,
    /// The task path associated with this failure.
    pub task_path: Option<String>,
    /// The stacktrace of this failure.
    pub stacktrace: Option<String>,
}

/// A test failure.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestFailure {
    /// Information to uniquely identify the test.
    pub id: TestId,
    /// Failure message.
    pub message: String,
    /// The failure stacktrace.
    pub stacktrace: Option<String>,
}

/// Information to uniquely identify a test.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestId {
    /// Work unit name that executed the test.
    pub work_unit_name: String,
    /// Suite name of the test (usually the class name).
    pub suite_name: String,
    /// Name of the test method. Null when the test is a suite.
    pub test_name: Option<String>,
}

impl TestId {
    /// Returns a display name for the test.
    pub fn display_name(&self) -> String {
        match &self.test_name {
            Some(name) => format!("{} > {}", self.suite_name, name),
            None => self.suite_name.clone(),
        }
    }
}
