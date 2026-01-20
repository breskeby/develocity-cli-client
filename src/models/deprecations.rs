//! Deprecation-related data models from the Develocity API.

use serde::{Deserialize, Serialize};

/// Gradle deprecations from `/api/builds/{id}/gradle-deprecations`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GradleDeprecations {
    /// List of deprecation entries. May be null if deprecation capturing is not supported.
    pub deprecations: Option<Vec<GradleDeprecationEntry>>,
}

/// A single deprecation entry.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleDeprecationEntry {
    /// Description of the deprecation.
    pub summary: String,
    /// Details about when the deprecated feature will be removed.
    pub removal_details: String,
    /// Advice on how to avoid using the deprecated feature.
    pub advice: Option<String>,
    /// URL to documentation about the deprecation.
    pub documentation_url: Option<String>,
    /// List of usages of this deprecation.
    pub usages: Vec<GradleDeprecationUsage>,
}

/// A single usage of a deprecation.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleDeprecationUsage {
    /// Contextual advice specific to this usage.
    pub contextual_advice: Option<String>,
    /// The owner of this deprecation usage.
    pub owner: GradleDeprecationOwner,
}

/// The owner of a deprecation usage.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GradleDeprecationOwner {
    /// The location of the deprecation usage (plugin id, task path, script path, etc.).
    pub location: Option<String>,
    /// The type of the owner.
    #[serde(rename = "type")]
    pub owner_type: DeprecationOwnerType,
}

/// The type of deprecation owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeprecationOwnerType {
    /// The owner is a plugin.
    Plugin,
    /// The owner is a script.
    Script,
    /// The owner is a task.
    Task,
    /// Unknown owner type.
    Unknown,
}

impl std::fmt::Display for DeprecationOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeprecationOwnerType::Plugin => write!(f, "plugin"),
            DeprecationOwnerType::Script => write!(f, "script"),
            DeprecationOwnerType::Task => write!(f, "task"),
            DeprecationOwnerType::Unknown => write!(f, "unknown"),
        }
    }
}
