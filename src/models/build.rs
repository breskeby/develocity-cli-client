//! Build-related data models from the Develocity API.

use serde::{Deserialize, Serialize};

/// Common build information from `/api/builds/{id}`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    /// The Build Scan ID.
    pub id: String,
    /// Unix timestamp (milliseconds) when the build became available in Develocity.
    pub available_at: i64,
    /// The build tool type (gradle, maven, bazel, sbt, etc.).
    pub build_tool_type: String,
    /// The version of the build tool.
    pub build_tool_version: String,
    /// The version of the Develocity build agent/plugin.
    pub build_agent_version: String,
}

impl Build {
    /// Returns true if this is a Gradle build.
    pub fn is_gradle(&self) -> bool {
        self.build_tool_type.eq_ignore_ascii_case("gradle")
    }
}

/// Gradle-specific build attributes from `/api/builds/{id}/gradle-attributes`.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleAttributes {
    /// The Build Scan ID.
    pub id: String,
    /// Unix timestamp (milliseconds) when the build started.
    pub build_start_time: i64,
    /// Duration of the build in milliseconds.
    pub build_duration: i64,
    /// The Gradle version used.
    pub gradle_version: String,
    /// The Develocity Gradle plugin version.
    pub plugin_version: String,
    /// The root project name. May be null for very early build failures.
    pub root_project_name: Option<String>,
    /// The list of requested tasks.
    pub requested_tasks: Vec<String>,
    /// True if the build failed.
    pub has_failed: bool,
    /// True if the build has at least one verification failure.
    /// Only set if the build failed.
    #[serde(default)]
    pub has_verification_failure: Option<bool>,
    /// True if the build has at least one non-verification failure.
    /// Only set if the build failed.
    #[serde(default)]
    pub has_non_verification_failure: Option<bool>,
    /// Build Scan tags.
    pub tags: Vec<String>,
    /// Custom values added to the Build Scan.
    pub values: Vec<BuildValue>,
    /// Links added to the Build Scan.
    pub links: Vec<BuildLink>,
    /// Environment information.
    pub environment: GradleEnvironment,
    /// Develocity-specific settings.
    #[serde(default)]
    pub develocity_settings: Option<DevelocitySettings>,
    /// Build options.
    #[serde(default)]
    pub build_options: Option<GradleBuildOptions>,
}

/// A custom key-value pair added to a Build Scan.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildValue {
    /// The name/key of the value.
    pub name: String,
    /// The value.
    pub value: String,
}

/// A link added to a Build Scan.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildLink {
    /// The label for the link.
    pub label: String,
    /// The URL.
    pub url: String,
}

/// Environment information for a Gradle build.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleEnvironment {
    /// The username of the OS account that executed the build.
    pub username: Option<String>,
    /// The operating system.
    pub operating_system: Option<String>,
    /// Number of CPU cores available.
    pub number_of_cpu_cores: Option<i32>,
    /// JRE version.
    pub jre_version: Option<String>,
    /// JVM version.
    pub jvm_version: Option<String>,
    /// Maximum JVM heap size in bytes.
    pub jvm_max_memory_heap_size: Option<i64>,
    /// JVM charset.
    pub jvm_charset: Option<String>,
    /// JVM locale.
    pub jvm_locale: Option<String>,
    /// Public hostname.
    pub public_hostname: Option<String>,
    /// Local hostname.
    pub local_hostname: Option<String>,
    /// Local IP addresses.
    #[serde(default)]
    pub local_ip_addresses: Vec<String>,
}

/// Develocity-specific settings for a build.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DevelocitySettings {
    /// Whether background Build Scan publication is enabled.
    pub background_publication_enabled: Option<bool>,
    /// Whether build output capturing is enabled.
    pub build_output_capturing_enabled: Option<bool>,
    /// Whether file fingerprint capturing is enabled.
    pub file_fingerprint_capturing_enabled: Option<bool>,
    /// Whether task inputs file capturing is enabled.
    pub task_inputs_file_capturing_enabled: Option<bool>,
    /// Whether test output capturing is enabled.
    pub test_output_capturing_enabled: Option<bool>,
    /// Whether resource usage capturing is enabled.
    pub resource_usage_capturing_enabled: Option<bool>,
}

/// Build options for a Gradle build.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleBuildOptions {
    /// Whether the build cache is enabled.
    pub build_cache_enabled: Option<bool>,
    /// Whether the configuration cache is enabled.
    pub configuration_cache_enabled: Option<bool>,
    /// Whether configuration on demand is enabled.
    pub configuration_on_demand_enabled: Option<bool>,
    /// Whether continuous build is enabled.
    pub continuous_build_enabled: Option<bool>,
    /// Whether continue on failure is enabled.
    pub continue_on_failure_enabled: Option<bool>,
    /// Whether the Gradle daemon is enabled.
    pub daemon_enabled: Option<bool>,
    /// Whether dry run mode is enabled.
    pub dry_run_enabled: Option<bool>,
    /// Excluded tasks.
    #[serde(default)]
    pub excluded_tasks: Vec<String>,
    /// Whether file system watching is enabled.
    pub file_system_watching_enabled: Option<bool>,
    /// Maximum number of Gradle workers.
    pub max_number_of_gradle_workers: Option<i32>,
    /// Whether offline mode is enabled.
    pub offline_mode_enabled: Option<bool>,
    /// Whether parallel project execution is enabled.
    pub parallel_project_execution_enabled: Option<bool>,
    /// Whether refresh dependencies is enabled.
    pub refresh_dependencies_enabled: Option<bool>,
    /// Whether rerun tasks is enabled.
    pub rerun_tasks_enabled: Option<bool>,
}
