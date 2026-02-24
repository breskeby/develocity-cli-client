//! Data models for the Gradle Build Cache Performance API response.
//!
//! Maps to `GET /api/builds/{id}/gradle-build-cache-performance`.
//! Contains per-task execution details including cache avoidance outcomes,
//! durations, and savings summaries.

use serde::Deserialize;
use std::fmt;

/// Top-level response from `/api/builds/{id}/gradle-build-cache-performance`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleBuildCachePerformance {
    /// Build scan ID.
    pub id: String,
    /// Total build time in milliseconds.
    pub build_time: i64,
    /// Effective task execution time in milliseconds (parallelism-adjusted).
    pub effective_task_execution_time: i64,
    /// Effective work unit execution time in milliseconds.
    pub effective_work_unit_execution_time: i64,
    /// Serial task execution time in milliseconds (sum of all task durations).
    pub serial_task_execution_time: i64,
    /// Serial work unit execution time in milliseconds.
    pub serial_work_unit_execution_time: i64,
    /// Serialization factor (serial / effective).
    pub serialization_factor: f64,
    /// Per-task execution entries.
    pub task_execution: Vec<TaskExecutionEntry>,
    /// Task fingerprinting summary (nullable).
    pub task_fingerprinting_summary: Option<FingerprintingSummary>,
    /// Work unit fingerprinting summary (nullable).
    pub work_unit_fingerprinting_summary: Option<FingerprintingSummary>,
    /// Deprecated avoidance savings summary (required by API, kept for deserialization).
    #[allow(dead_code)]
    pub avoidance_savings_summary: AvoidanceSavingsSummary,
    /// Task avoidance savings summary.
    pub task_avoidance_savings_summary: AvoidanceSavingsSummary,
    /// Work unit avoidance savings summary.
    pub work_unit_avoidance_savings_summary: AvoidanceSavingsSummary,
    /// Build cache configuration (nullable).
    pub build_caches: Option<BuildCaches>,
}

/// Per-task execution entry with cache performance details.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskExecutionEntry {
    /// Gradle task path (e.g., ":app:compileJava").
    pub task_path: String,
    /// Fully qualified task type.
    pub task_type: String,
    /// How this task was avoided or executed.
    pub avoidance_outcome: AvoidanceOutcome,
    /// Task duration in milliseconds.
    pub duration: i64,
    /// Fingerprinting duration in milliseconds (nullable).
    pub fingerprinting_duration: Option<i64>,
    /// Time saved by avoidance in milliseconds (nullable).
    pub avoidance_savings: Option<i64>,
    /// Why the task is not cacheable (nullable).
    pub non_cacheability_category: Option<NonCacheabilityCategory>,
    /// Human-readable reason for non-cacheability (nullable).
    pub non_cacheability_reason: Option<String>,
    /// Reason the task was skipped (nullable).
    pub skip_reason_message: Option<String>,
    /// Size of the cache artifact in bytes (nullable).
    pub cache_artifact_size: Option<i64>,
    /// Why the cache artifact was rejected (nullable).
    pub cache_artifact_rejected_reason: Option<CacheArtifactRejectedReason>,
    /// Cache key hash (nullable).
    pub cache_key: Option<String>,
    /// Whether this task failed.
    pub has_failed: bool,
}

/// How a task was avoided or executed.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AvoidanceOutcome {
    /// Task was up-to-date.
    AvoidedUpToDate,
    /// Task output was found in the local cache.
    AvoidedFromLocalCache,
    /// Task output was found in the remote cache.
    AvoidedFromRemoteCache,
    /// Task was executed and is cacheable.
    ExecutedCacheable,
    /// Task was executed and is not cacheable.
    ExecutedNotCacheable,
    /// Task was executed with unknown cacheability.
    ExecutedUnknownCacheability,
    /// Task was avoided for an unknown reason.
    AvoidedUnknownReason,
    /// Lifecycle task (no actions).
    Lifecycle,
    /// Task had no source inputs.
    #[serde(rename = "no-source")]
    NoSource,
    /// Task was skipped.
    Skipped,
}

impl fmt::Display for AvoidanceOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AvoidanceOutcome::AvoidedUpToDate => write!(f, "UP-TO-DATE"),
            AvoidanceOutcome::AvoidedFromLocalCache => write!(f, "FROM-CACHE (local)"),
            AvoidanceOutcome::AvoidedFromRemoteCache => write!(f, "FROM-CACHE (remote)"),
            AvoidanceOutcome::ExecutedCacheable => write!(f, "EXECUTED (cacheable)"),
            AvoidanceOutcome::ExecutedNotCacheable => write!(f, "EXECUTED (not cacheable)"),
            AvoidanceOutcome::ExecutedUnknownCacheability => {
                write!(f, "EXECUTED (unknown cacheability)")
            }
            AvoidanceOutcome::AvoidedUnknownReason => write!(f, "AVOIDED (unknown)"),
            AvoidanceOutcome::Lifecycle => write!(f, "LIFECYCLE"),
            AvoidanceOutcome::NoSource => write!(f, "NO-SOURCE"),
            AvoidanceOutcome::Skipped => write!(f, "SKIPPED"),
        }
    }
}

/// Why a task is not cacheable.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NonCacheabilityCategory {
    BuildCacheNotEnabled,
    #[serde(rename = "cache-if_condition_not_matched")]
    CacheIfConditionNotMatched,
    DisabledToEnsureCorrectness,
    #[serde(rename = "do-not-cache-if_condition_matched")]
    DoNotCacheIfConditionMatched,
    MultipleOutputsDeclared,
    NoOutputsDeclared,
    NonCacheableInputs,
    NonCacheableTaskAction,
    NonCacheableTaskImplementation,
    NonCacheableTreeOutput,
    OverlappingOutputs,
    TaskHasNoActions,
    TaskOutputCachingNotEnabled,
    Unknown,
}

impl fmt::Display for NonCacheabilityCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NonCacheabilityCategory::BuildCacheNotEnabled => write!(f, "build cache not enabled"),
            NonCacheabilityCategory::CacheIfConditionNotMatched => {
                write!(f, "cache-if condition not matched")
            }
            NonCacheabilityCategory::DisabledToEnsureCorrectness => {
                write!(f, "disabled to ensure correctness")
            }
            NonCacheabilityCategory::DoNotCacheIfConditionMatched => {
                write!(f, "do-not-cache-if condition matched")
            }
            NonCacheabilityCategory::MultipleOutputsDeclared => {
                write!(f, "multiple outputs declared")
            }
            NonCacheabilityCategory::NoOutputsDeclared => write!(f, "no outputs declared"),
            NonCacheabilityCategory::NonCacheableInputs => write!(f, "non-cacheable inputs"),
            NonCacheabilityCategory::NonCacheableTaskAction => {
                write!(f, "non-cacheable task action")
            }
            NonCacheabilityCategory::NonCacheableTaskImplementation => {
                write!(f, "non-cacheable task implementation")
            }
            NonCacheabilityCategory::NonCacheableTreeOutput => {
                write!(f, "non-cacheable tree output")
            }
            NonCacheabilityCategory::OverlappingOutputs => write!(f, "overlapping outputs"),
            NonCacheabilityCategory::TaskHasNoActions => write!(f, "task has no actions"),
            NonCacheabilityCategory::TaskOutputCachingNotEnabled => {
                write!(f, "task output caching not enabled")
            }
            NonCacheabilityCategory::Unknown => write!(f, "unknown"),
        }
    }
}

/// Why a cache artifact was rejected.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CacheArtifactRejectedReason {
    /// Artifact exceeded the maximum allowed size.
    ArtifactExceedsMaximumSize,
    /// Artifact was rejected for an unknown reason.
    Unknown,
}

impl fmt::Display for CacheArtifactRejectedReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheArtifactRejectedReason::ArtifactExceedsMaximumSize => {
                write!(f, "artifact exceeds maximum size")
            }
            CacheArtifactRejectedReason::Unknown => write!(f, "unknown"),
        }
    }
}

/// Avoidance savings summary (task or work unit level).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvoidanceSavingsSummary {
    /// Total avoidance savings in milliseconds.
    pub total: i64,
    /// Avoidance savings ratio (0.0 to 1.0).
    pub ratio: f64,
    /// Savings from up-to-date avoidance in milliseconds.
    pub up_to_date: i64,
    /// Savings from local build cache in milliseconds.
    pub local_build_cache: i64,
    /// Savings from remote build cache in milliseconds.
    pub remote_build_cache: i64,
}

/// Fingerprinting summary (task or work unit level).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintingSummary {
    /// Number of tasks fingerprinted.
    pub count: i32,
    /// Total serial fingerprinting duration in milliseconds.
    pub serial_duration: i64,
}

/// Build cache configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildCaches {
    /// Local build cache configuration (nullable).
    pub local: Option<LocalBuildCache>,
    /// Remote build cache configuration (nullable).
    pub remote: Option<RemoteBuildCache>,
    /// Cache overhead timings (nullable).
    pub overhead: Option<CacheOverhead>,
}

/// Local build cache configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalBuildCache {
    /// Whether the local cache is enabled.
    pub is_enabled: bool,
    /// Whether push to local cache is enabled (nullable, absent when disabled).
    pub is_push_enabled: Option<bool>,
    /// Whether the local cache can be used for remote storage.
    pub is_store_enabled_for_remote_build_cache: Option<bool>,
}

/// Remote build cache configuration.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteBuildCache {
    /// Whether the remote cache is enabled.
    pub is_enabled: bool,
    /// Whether push to remote cache is enabled (nullable, absent when disabled).
    pub is_push_enabled: Option<bool>,
    /// Remote cache URL (nullable).
    pub url: Option<String>,
}

/// Cache overhead timings.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheOverhead {
    /// Upload overhead in milliseconds.
    pub uploading: i64,
    /// Download overhead in milliseconds.
    pub downloading: i64,
    /// Pack overhead in milliseconds.
    pub packing: i64,
    /// Unpack overhead in milliseconds.
    pub unpacking: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avoidance_outcome_display() {
        assert_eq!(AvoidanceOutcome::AvoidedUpToDate.to_string(), "UP-TO-DATE");
        assert_eq!(
            AvoidanceOutcome::AvoidedFromLocalCache.to_string(),
            "FROM-CACHE (local)"
        );
        assert_eq!(
            AvoidanceOutcome::AvoidedFromRemoteCache.to_string(),
            "FROM-CACHE (remote)"
        );
        assert_eq!(
            AvoidanceOutcome::ExecutedCacheable.to_string(),
            "EXECUTED (cacheable)"
        );
        assert_eq!(
            AvoidanceOutcome::ExecutedNotCacheable.to_string(),
            "EXECUTED (not cacheable)"
        );
        assert_eq!(AvoidanceOutcome::Lifecycle.to_string(), "LIFECYCLE");
        assert_eq!(AvoidanceOutcome::NoSource.to_string(), "NO-SOURCE");
        assert_eq!(AvoidanceOutcome::Skipped.to_string(), "SKIPPED");
    }

    #[test]
    fn test_non_cacheability_category_display() {
        assert_eq!(
            NonCacheabilityCategory::BuildCacheNotEnabled.to_string(),
            "build cache not enabled"
        );
        assert_eq!(
            NonCacheabilityCategory::TaskOutputCachingNotEnabled.to_string(),
            "task output caching not enabled"
        );
        assert_eq!(NonCacheabilityCategory::Unknown.to_string(), "unknown");
    }

    #[test]
    fn test_avoidance_outcome_deserialize() {
        let json = r#""avoided_up_to_date""#;
        let outcome: AvoidanceOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, AvoidanceOutcome::AvoidedUpToDate);

        let json = r#""no-source""#;
        let outcome: AvoidanceOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, AvoidanceOutcome::NoSource);

        let json = r#""executed_cacheable""#;
        let outcome: AvoidanceOutcome = serde_json::from_str(json).unwrap();
        assert_eq!(outcome, AvoidanceOutcome::ExecutedCacheable);
    }

    #[test]
    fn test_non_cacheability_category_deserialize() {
        let json = r#""cache-if_condition_not_matched""#;
        let cat: NonCacheabilityCategory = serde_json::from_str(json).unwrap();
        assert_eq!(cat, NonCacheabilityCategory::CacheIfConditionNotMatched);

        let json = r#""do-not-cache-if_condition_matched""#;
        let cat: NonCacheabilityCategory = serde_json::from_str(json).unwrap();
        assert_eq!(cat, NonCacheabilityCategory::DoNotCacheIfConditionMatched);
    }

    #[test]
    fn test_task_execution_entry_deserialize() {
        let json = r#"{
            "taskPath": ":app:compileJava",
            "taskType": "org.gradle.api.tasks.compile.JavaCompile",
            "avoidanceOutcome": "executed_cacheable",
            "duration": 1234,
            "fingerprintingDuration": 100,
            "avoidanceSavings": null,
            "nonCacheabilityCategory": null,
            "nonCacheabilityReason": null,
            "skipReasonMessage": null,
            "cacheArtifactSize": 5678,
            "cacheArtifactRejectedReason": null,
            "cacheKey": "abc123",
            "hasFailed": false
        }"#;

        let entry: TaskExecutionEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.task_path, ":app:compileJava");
        assert_eq!(entry.task_type, "org.gradle.api.tasks.compile.JavaCompile");
        assert_eq!(entry.avoidance_outcome, AvoidanceOutcome::ExecutedCacheable);
        assert_eq!(entry.duration, 1234);
        assert_eq!(entry.fingerprinting_duration, Some(100));
        assert_eq!(entry.avoidance_savings, None);
        assert_eq!(entry.cache_artifact_size, Some(5678));
        assert_eq!(entry.cache_key, Some("abc123".to_string()));
        assert!(!entry.has_failed);
    }

    #[test]
    fn test_avoidance_savings_summary_deserialize() {
        let json = r#"{
            "total": 5000,
            "ratio": 0.42,
            "upToDate": 3000,
            "localBuildCache": 1500,
            "remoteBuildCache": 500
        }"#;

        let summary: AvoidanceSavingsSummary = serde_json::from_str(json).unwrap();
        assert_eq!(summary.total, 5000);
        assert!((summary.ratio - 0.42).abs() < 0.001);
        assert_eq!(summary.up_to_date, 3000);
        assert_eq!(summary.local_build_cache, 1500);
        assert_eq!(summary.remote_build_cache, 500);
    }
}
