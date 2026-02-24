//! Test-related data models from the Develocity API.
//!
//! These models match the `/api/tests/build/{id}` endpoint response
//! (BuildTestsResponse schema from the Develocity API specification).

use serde::{Deserialize, Serialize};

/// Test outcome enumeration matching the Develocity API TestOutcome.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TestOutcome {
    Passed,
    Failed,
    Skipped,
    Flaky,
    NotSelected,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for TestOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestOutcome::Passed => write!(f, "passed"),
            TestOutcome::Failed => write!(f, "failed"),
            TestOutcome::Skipped => write!(f, "skipped"),
            TestOutcome::Flaky => write!(f, "flaky"),
            TestOutcome::NotSelected => write!(f, "notSelected"),
            TestOutcome::Unknown => write!(f, "unknown"),
        }
    }
}

impl std::str::FromStr for TestOutcome {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "passed" => Ok(TestOutcome::Passed),
            "failed" => Ok(TestOutcome::Failed),
            "skipped" => Ok(TestOutcome::Skipped),
            "flaky" => Ok(TestOutcome::Flaky),
            "notselected" | "not_selected" | "not-selected" => Ok(TestOutcome::NotSelected),
            other => Err(format!(
                "Invalid test outcome '{}'. Valid values: passed, failed, skipped, flaky, notSelected",
                other
            )),
        }
    }
}

/// Parse a comma-separated string of test outcomes.
///
/// # Examples
/// ```
/// use dvcli::models::tests::parse_test_outcomes;
///
/// let outcomes = parse_test_outcomes("failed,flaky").unwrap();
/// assert_eq!(outcomes.len(), 2);
/// ```
pub fn parse_test_outcomes(s: &str) -> std::result::Result<Vec<TestOutcome>, String> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(Vec::new());
    }
    s.split(',')
        .map(|part| part.trim().parse::<TestOutcome>())
        .collect()
}

/// Work unit outcome enumeration (TestWorkUnitOutcome).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TestWorkUnitOutcome {
    Passed,
    Failed,
    Skipped,
    Flaky,
    TimedOut,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for TestWorkUnitOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestWorkUnitOutcome::Passed => write!(f, "passed"),
            TestWorkUnitOutcome::Failed => write!(f, "failed"),
            TestWorkUnitOutcome::Skipped => write!(f, "skipped"),
            TestWorkUnitOutcome::Flaky => write!(f, "flaky"),
            TestWorkUnitOutcome::TimedOut => write!(f, "timed out"),
            TestWorkUnitOutcome::Unknown => write!(f, "unknown"),
        }
    }
}

/// Response from `GET /api/tests/build/{id}` (BuildTestsResponse).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTestsResponse {
    /// Summary of test execution data for the build.
    pub summary: BuildTestsSummary,
    /// List of test work units executed in the build.
    pub work_units: Vec<BuildTestWorkUnit>,
}

/// Summary of test execution data for a build (BuildTestsSummary).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTestsSummary {
    /// Duration information.
    pub duration: BuildTestsDuration,
    /// Outcome distribution for test cases.
    pub test_cases_outcome_distribution: TestOutcomeDistribution,
    /// Outcome distribution for test containers.
    pub test_containers_outcome_distribution: TestOutcomeDistribution,
}

/// Duration information at the build summary level.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTestsDuration {
    /// Sum of elapsed wall-clock time in milliseconds of work units with tests.
    pub total: i64,
    /// Sum of time in milliseconds if tests ran sequentially. May be absent.
    pub serial: Option<i64>,
}

/// Distribution of test outcomes (TestOutcomeDistribution).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestOutcomeDistribution {
    /// Total number of outcomes.
    pub total: i32,
    /// Number of passed outcomes.
    pub passed: i32,
    /// Number of failed outcomes.
    pub failed: i32,
    /// Number of skipped outcomes.
    pub skipped: i32,
    /// Number of flaky outcomes.
    pub flaky: i32,
    /// Number of "not selected" outcomes.
    pub not_selected: i32,
}

/// A work unit that ran tests in a build (BuildTestWorkUnit).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTestWorkUnit {
    /// Name of the work unit (e.g., ":test", ":app:test").
    pub name: String,
    /// Duration information for this work unit.
    pub duration: BuildTestOrContainerDuration,
    /// Outcome of this work unit.
    pub outcome: TestWorkUnitOutcome,
    /// List of test containers or test cases executed in this work unit.
    pub tests: Vec<BuildTestOrContainer>,
}

/// Various types of test durations (BuildTestOrContainerDuration).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTestOrContainerDuration {
    /// Elapsed wall-clock time in milliseconds. May be absent.
    pub total: Option<i64>,
    /// Time spent in setup/cleanup (containers only). Absent for test cases.
    pub own: Option<i64>,
    /// Sum of time if tests ran sequentially. Absent for test cases.
    pub serial: Option<i64>,
}

/// A test or test container in the hierarchy (BuildTestOrContainer).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTestOrContainer {
    /// Name of the test or test container.
    pub name: String,
    /// Duration information.
    pub duration: BuildTestOrContainerDuration,
    /// Outcome information.
    pub outcome: BuildTestOrContainerOutcome,
    /// Individual test executions (a test can run multiple times with retries).
    pub executions: Vec<BuildTestOrContainerExecution>,
    /// Child test containers or test cases.
    pub children: Vec<BuildTestOrContainer>,
}

/// Outcome information for a test or container (BuildTestOrContainerOutcome).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTestOrContainerOutcome {
    /// Overall outcome of the test or container.
    pub overall: TestOutcome,
    /// Own outcome (for containers: setup/cleanup outcome).
    pub own: Option<TestOutcome>,
    /// Children outcome (for containers: aggregate of children).
    pub children: Option<TestOutcome>,
}

/// A single execution of a test or container (BuildTestOrContainerExecution).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTestOrContainerExecution {
    /// Duration of this execution.
    pub duration: BuildTestOrContainerDuration,
    /// Outcome of this execution.
    pub outcome: BuildTestOrContainerOutcome,
}

impl BuildTestOrContainer {
    /// Returns true if this is a leaf test case (no children).
    pub fn is_test_case(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns true if this is a container (has children).
    pub fn is_container(&self) -> bool {
        !self.children.is_empty()
    }
}

/// A flattened test case for output purposes.
/// Created by walking the hierarchical tree and extracting leaf test cases.
#[derive(Debug, Clone)]
pub struct FlattenedTestCase {
    /// Work unit name (e.g., ":test").
    pub work_unit: String,
    /// Fully qualified container name (e.g., "org.example.TestClass").
    pub container_name: String,
    /// Test method name (leaf name).
    pub test_name: String,
    /// Overall outcome.
    pub outcome: TestOutcome,
    /// Duration in milliseconds.
    pub duration_ms: i64,
    /// Number of executions (retries).
    pub execution_count: usize,
}

impl BuildTestsResponse {
    /// Flatten the hierarchical test tree into a list of leaf test cases.
    ///
    /// This walks the tree and extracts all leaf nodes (test cases with no children),
    /// building up the fully qualified container name from the path.
    pub fn flatten_test_cases(&self) -> Vec<FlattenedTestCase> {
        let mut result = Vec::new();
        for wu in &self.work_units {
            for test in &wu.tests {
                Self::flatten_recursive(&wu.name, "", test, &mut result);
            }
        }
        result
    }

    fn flatten_recursive(
        work_unit: &str,
        parent_path: &str,
        node: &BuildTestOrContainer,
        result: &mut Vec<FlattenedTestCase>,
    ) {
        if node.is_test_case() {
            // This is a leaf test case
            let container_name = if parent_path.is_empty() {
                // The node itself is both container and test
                node.name.clone()
            } else {
                parent_path.to_string()
            };

            result.push(FlattenedTestCase {
                work_unit: work_unit.to_string(),
                container_name,
                test_name: node.name.clone(),
                outcome: node.outcome.overall.clone(),
                duration_ms: node.duration.total.unwrap_or(0),
                execution_count: node.executions.len(),
            });
        } else {
            // This is a container — recurse into children
            // Build path: skip framework-level names like "JUnit Jupiter" / "JUnit Vintage"
            let current_path = if parent_path.is_empty() {
                node.name.clone()
            } else {
                // For nested containers, concatenate with "."
                // But only if the child name looks like a class/package segment
                format!("{}.{}", parent_path, node.name)
            };

            for child in &node.children {
                Self::flatten_recursive(work_unit, &current_path, child, result);
            }
        }
    }
}

// Backward compatibility: re-export the main response type under a shorter alias
pub type GradleTests = BuildTestsResponse;

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
    fn test_outcome_deserialize_flaky() {
        let json = r#""flaky""#;
        let outcome: TestOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, TestOutcome::Flaky);
    }

    #[test]
    fn test_outcome_deserialize_not_selected() {
        let json = r#""notSelected""#;
        let outcome: TestOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, TestOutcome::NotSelected);
    }

    #[test]
    fn test_outcome_deserialize_unknown_value() {
        let json = r#""some_future_status""#;
        let outcome: TestOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, TestOutcome::Unknown);
    }

    #[test]
    fn test_outcome_display() {
        assert_eq!(TestOutcome::Passed.to_string(), "passed");
        assert_eq!(TestOutcome::Failed.to_string(), "failed");
        assert_eq!(TestOutcome::Skipped.to_string(), "skipped");
        assert_eq!(TestOutcome::Flaky.to_string(), "flaky");
        assert_eq!(TestOutcome::NotSelected.to_string(), "notSelected");
        assert_eq!(TestOutcome::Unknown.to_string(), "unknown");
    }

    #[test]
    fn test_outcome_from_str() {
        assert_eq!(
            "passed".parse::<TestOutcome>().unwrap(),
            TestOutcome::Passed
        );
        assert_eq!(
            "FAILED".parse::<TestOutcome>().unwrap(),
            TestOutcome::Failed
        );
        assert_eq!(
            "Skipped".parse::<TestOutcome>().unwrap(),
            TestOutcome::Skipped
        );
        assert_eq!("flaky".parse::<TestOutcome>().unwrap(), TestOutcome::Flaky);
        assert_eq!(
            "notSelected".parse::<TestOutcome>().unwrap(),
            TestOutcome::NotSelected
        );
        assert_eq!(
            "not_selected".parse::<TestOutcome>().unwrap(),
            TestOutcome::NotSelected
        );
        assert_eq!(
            "not-selected".parse::<TestOutcome>().unwrap(),
            TestOutcome::NotSelected
        );
        assert!("invalid".parse::<TestOutcome>().is_err());
    }

    #[test]
    fn test_parse_test_outcomes() {
        use super::parse_test_outcomes;

        let outcomes = parse_test_outcomes("failed,flaky").unwrap();
        assert_eq!(outcomes.len(), 2);
        assert_eq!(outcomes[0], TestOutcome::Failed);
        assert_eq!(outcomes[1], TestOutcome::Flaky);

        let outcomes = parse_test_outcomes("passed").unwrap();
        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0], TestOutcome::Passed);

        let outcomes = parse_test_outcomes("").unwrap();
        assert!(outcomes.is_empty());

        assert!(parse_test_outcomes("failed,invalid").is_err());
    }

    #[test]
    fn test_outcome_serialize_roundtrip() {
        for outcome in [
            TestOutcome::Passed,
            TestOutcome::Failed,
            TestOutcome::Skipped,
            TestOutcome::Flaky,
            TestOutcome::NotSelected,
        ] {
            let json = serde_json::to_string(&outcome).unwrap();
            let deserialized: TestOutcome = serde_json::from_str(&json).unwrap();
            assert_eq!(outcome, deserialized);
        }
    }

    // ==================== TestWorkUnitOutcome Tests ====================

    #[test]
    fn test_work_unit_outcome_display() {
        assert_eq!(TestWorkUnitOutcome::Passed.to_string(), "passed");
        assert_eq!(TestWorkUnitOutcome::Failed.to_string(), "failed");
        assert_eq!(TestWorkUnitOutcome::Skipped.to_string(), "skipped");
        assert_eq!(TestWorkUnitOutcome::Flaky.to_string(), "flaky");
        assert_eq!(TestWorkUnitOutcome::TimedOut.to_string(), "timed out");
        assert_eq!(TestWorkUnitOutcome::Unknown.to_string(), "unknown");
    }

    #[test]
    fn test_work_unit_outcome_deserialize_timed_out() {
        let json = r#""timedOut""#;
        let outcome: TestWorkUnitOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, TestWorkUnitOutcome::TimedOut);
    }

    // ==================== BuildTestsResponse Deserialization ====================

    #[test]
    fn test_response_deserialize_minimal() {
        let json = r#"{
            "summary": {
                "duration": { "total": 1000 },
                "testCasesOutcomeDistribution": {
                    "total": 2, "passed": 1, "failed": 1, "skipped": 0, "flaky": 0, "notSelected": 0
                },
                "testContainersOutcomeDistribution": {
                    "total": 1, "passed": 1, "failed": 0, "skipped": 0, "flaky": 0, "notSelected": 0
                }
            },
            "workUnits": []
        }"#;

        let response: BuildTestsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.summary.duration.total, 1000);
        assert!(response.summary.duration.serial.is_none());
        assert_eq!(response.summary.test_cases_outcome_distribution.total, 2);
        assert_eq!(response.summary.test_cases_outcome_distribution.passed, 1);
        assert_eq!(response.summary.test_cases_outcome_distribution.failed, 1);
        assert!(response.work_units.is_empty());
    }

    #[test]
    fn test_response_deserialize_full_hierarchy() {
        let json = r#"{
            "summary": {
                "duration": { "total": 150, "serial": 150 },
                "testCasesOutcomeDistribution": {
                    "total": 3, "passed": 1, "failed": 1, "skipped": 0, "flaky": 1, "notSelected": 0
                },
                "testContainersOutcomeDistribution": {
                    "total": 2, "passed": 2, "failed": 0, "skipped": 0, "flaky": 0, "notSelected": 0
                }
            },
            "workUnits": [
                {
                    "name": ":test",
                    "duration": { "total": 150, "own": 50, "serial": 150 },
                    "outcome": "failed",
                    "tests": [
                        {
                            "name": "org.example.TestContainer",
                            "duration": { "total": 100, "own": 10, "serial": 100 },
                            "outcome": { "overall": "failed", "own": "passed", "children": "failed" },
                            "executions": [
                                {
                                    "duration": { "total": 100 },
                                    "outcome": { "overall": "failed" }
                                }
                            ],
                            "children": [
                                {
                                    "name": "successfulTest",
                                    "duration": { "total": 30 },
                                    "outcome": { "overall": "passed" },
                                    "executions": [
                                        { "duration": { "total": 30 }, "outcome": { "overall": "passed" } }
                                    ],
                                    "children": []
                                },
                                {
                                    "name": "failingTest",
                                    "duration": { "total": 20 },
                                    "outcome": { "overall": "failed" },
                                    "executions": [
                                        { "duration": { "total": 10 }, "outcome": { "overall": "failed" } },
                                        { "duration": { "total": 10 }, "outcome": { "overall": "failed" } }
                                    ],
                                    "children": []
                                },
                                {
                                    "name": "flakyTest",
                                    "duration": { "total": 40 },
                                    "outcome": { "overall": "flaky" },
                                    "executions": [
                                        { "duration": { "total": 20 }, "outcome": { "overall": "failed" } },
                                        { "duration": { "total": 20 }, "outcome": { "overall": "passed" } }
                                    ],
                                    "children": []
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let response: BuildTestsResponse = serde_json::from_str(json).unwrap();

        // Check summary
        assert_eq!(response.summary.duration.total, 150);
        assert_eq!(response.summary.duration.serial, Some(150));
        assert_eq!(response.summary.test_cases_outcome_distribution.total, 3);
        assert_eq!(response.summary.test_cases_outcome_distribution.flaky, 1);

        // Check work units
        assert_eq!(response.work_units.len(), 1);
        let wu = &response.work_units[0];
        assert_eq!(wu.name, ":test");
        assert_eq!(wu.outcome, TestWorkUnitOutcome::Failed);

        // Check test hierarchy
        assert_eq!(wu.tests.len(), 1);
        let container = &wu.tests[0];
        assert_eq!(container.name, "org.example.TestContainer");
        assert!(container.is_container());
        assert_eq!(container.children.len(), 3);

        // Check leaf test cases
        let successful = &container.children[0];
        assert_eq!(successful.name, "successfulTest");
        assert!(successful.is_test_case());
        assert_eq!(successful.outcome.overall, TestOutcome::Passed);

        let failing = &container.children[1];
        assert_eq!(failing.name, "failingTest");
        assert_eq!(failing.outcome.overall, TestOutcome::Failed);
        assert_eq!(failing.executions.len(), 2); // Two retries

        let flaky = &container.children[2];
        assert_eq!(flaky.name, "flakyTest");
        assert_eq!(flaky.outcome.overall, TestOutcome::Flaky);
    }

    // ==================== Flatten Tests ====================

    #[test]
    fn test_flatten_test_cases() {
        let response = BuildTestsResponse {
            summary: BuildTestsSummary {
                duration: BuildTestsDuration {
                    total: 100,
                    serial: Some(100),
                },
                test_cases_outcome_distribution: TestOutcomeDistribution {
                    total: 2,
                    passed: 1,
                    failed: 1,
                    skipped: 0,
                    flaky: 0,
                    not_selected: 0,
                },
                test_containers_outcome_distribution: TestOutcomeDistribution {
                    total: 1,
                    passed: 1,
                    failed: 0,
                    skipped: 0,
                    flaky: 0,
                    not_selected: 0,
                },
            },
            work_units: vec![BuildTestWorkUnit {
                name: ":test".to_string(),
                duration: BuildTestOrContainerDuration {
                    total: Some(100),
                    own: Some(10),
                    serial: Some(100),
                },
                outcome: TestWorkUnitOutcome::Failed,
                tests: vec![BuildTestOrContainer {
                    name: "org.example.MyTest".to_string(),
                    duration: BuildTestOrContainerDuration {
                        total: Some(90),
                        own: Some(5),
                        serial: Some(90),
                    },
                    outcome: BuildTestOrContainerOutcome {
                        overall: TestOutcome::Failed,
                        own: Some(TestOutcome::Passed),
                        children: Some(TestOutcome::Failed),
                    },
                    executions: vec![],
                    children: vec![
                        BuildTestOrContainer {
                            name: "testPass".to_string(),
                            duration: BuildTestOrContainerDuration {
                                total: Some(40),
                                own: None,
                                serial: None,
                            },
                            outcome: BuildTestOrContainerOutcome {
                                overall: TestOutcome::Passed,
                                own: None,
                                children: None,
                            },
                            executions: vec![BuildTestOrContainerExecution {
                                duration: BuildTestOrContainerDuration {
                                    total: Some(40),
                                    own: None,
                                    serial: None,
                                },
                                outcome: BuildTestOrContainerOutcome {
                                    overall: TestOutcome::Passed,
                                    own: None,
                                    children: None,
                                },
                            }],
                            children: vec![],
                        },
                        BuildTestOrContainer {
                            name: "testFail".to_string(),
                            duration: BuildTestOrContainerDuration {
                                total: Some(45),
                                own: None,
                                serial: None,
                            },
                            outcome: BuildTestOrContainerOutcome {
                                overall: TestOutcome::Failed,
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
                                        total: Some(25),
                                        own: None,
                                        serial: None,
                                    },
                                    outcome: BuildTestOrContainerOutcome {
                                        overall: TestOutcome::Failed,
                                        own: None,
                                        children: None,
                                    },
                                },
                            ],
                            children: vec![],
                        },
                    ],
                }],
            }],
        };

        let flattened = response.flatten_test_cases();
        assert_eq!(flattened.len(), 2);

        assert_eq!(flattened[0].work_unit, ":test");
        assert_eq!(flattened[0].container_name, "org.example.MyTest");
        assert_eq!(flattened[0].test_name, "testPass");
        assert_eq!(flattened[0].outcome, TestOutcome::Passed);
        assert_eq!(flattened[0].duration_ms, 40);
        assert_eq!(flattened[0].execution_count, 1);

        assert_eq!(flattened[1].test_name, "testFail");
        assert_eq!(flattened[1].outcome, TestOutcome::Failed);
        assert_eq!(flattened[1].duration_ms, 45);
        assert_eq!(flattened[1].execution_count, 2);
    }

    #[test]
    fn test_flatten_empty_work_units() {
        let response = BuildTestsResponse {
            summary: BuildTestsSummary {
                duration: BuildTestsDuration {
                    total: 0,
                    serial: None,
                },
                test_cases_outcome_distribution: TestOutcomeDistribution {
                    total: 0,
                    passed: 0,
                    failed: 0,
                    skipped: 0,
                    flaky: 0,
                    not_selected: 0,
                },
                test_containers_outcome_distribution: TestOutcomeDistribution {
                    total: 0,
                    passed: 0,
                    failed: 0,
                    skipped: 0,
                    flaky: 0,
                    not_selected: 0,
                },
            },
            work_units: vec![],
        };

        let flattened = response.flatten_test_cases();
        assert!(flattened.is_empty());
    }

    // ==================== TestOutcomeDistribution Tests ====================

    #[test]
    fn test_outcome_distribution_deserialize() {
        let json = r#"{
            "total": 100,
            "passed": 80,
            "failed": 5,
            "skipped": 10,
            "flaky": 3,
            "notSelected": 2
        }"#;

        let dist: TestOutcomeDistribution = serde_json::from_str(json).unwrap();
        assert_eq!(dist.total, 100);
        assert_eq!(dist.passed, 80);
        assert_eq!(dist.failed, 5);
        assert_eq!(dist.skipped, 10);
        assert_eq!(dist.flaky, 3);
        assert_eq!(dist.not_selected, 2);
    }

    // ==================== Real API Response Test ====================

    #[test]
    fn test_deserialize_real_api_response_shape() {
        // Based on the actual response from the Develocity API
        let json = r#"{
            "summary": {
                "duration": {
                    "total": 1153868,
                    "serial": 1153868
                },
                "testCasesOutcomeDistribution": {
                    "passed": 50,
                    "failed": 0,
                    "skipped": 1,
                    "flaky": 0,
                    "notSelected": 0,
                    "total": 51
                },
                "testContainersOutcomeDistribution": {
                    "passed": 10,
                    "failed": 0,
                    "skipped": 0,
                    "flaky": 0,
                    "notSelected": 0,
                    "total": 10
                }
            },
            "workUnits": [
                {
                    "name": ":x-pack:plugin:transform:qa:multi-node-tests:javaRestTest",
                    "duration": { "total": 1039426, "own": 79827, "serial": 1039426 },
                    "outcome": "passed",
                    "tests": [
                        {
                            "name": "org.elasticsearch.xpack.transform.integration.LatestIT",
                            "duration": { "total": 28004, "own": 1768, "serial": 28004 },
                            "outcome": { "overall": "passed", "own": "passed", "children": "passed" },
                            "executions": [
                                {
                                    "duration": { "total": 28004, "own": 1768, "serial": 28004 },
                                    "outcome": { "overall": "passed", "own": "passed", "children": "passed" }
                                }
                            ],
                            "children": [
                                {
                                    "name": "testLatest",
                                    "duration": { "total": 12814, "serial": 12814 },
                                    "outcome": { "overall": "passed" },
                                    "executions": [
                                        { "duration": { "total": 12814, "serial": 12814 }, "outcome": { "overall": "passed" } }
                                    ],
                                    "children": []
                                },
                                {
                                    "name": "testLatestPreview",
                                    "duration": { "total": 13422, "serial": 13422 },
                                    "outcome": { "overall": "passed" },
                                    "executions": [
                                        { "duration": { "total": 13422, "serial": 13422 }, "outcome": { "overall": "passed" } }
                                    ],
                                    "children": []
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;

        let response: BuildTestsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.summary.test_cases_outcome_distribution.total, 51);
        assert_eq!(response.work_units.len(), 1);

        let wu = &response.work_units[0];
        assert_eq!(
            wu.name,
            ":x-pack:plugin:transform:qa:multi-node-tests:javaRestTest"
        );

        let flattened = response.flatten_test_cases();
        assert_eq!(flattened.len(), 2);
        assert_eq!(
            flattened[0].container_name,
            "org.elasticsearch.xpack.transform.integration.LatestIT"
        );
        assert_eq!(flattened[0].test_name, "testLatest");
        assert_eq!(flattened[1].test_name, "testLatestPreview");
    }
}
