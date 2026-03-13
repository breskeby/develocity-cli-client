//! Data models for the Gradle Dependencies API response.
//!
//! Maps to `GET /api/builds/{id}/gradle-dependencies`.

use serde::Deserialize;
use std::collections::HashMap;

/// Top-level response from `/api/builds/{id}/gradle-dependencies`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleDependencies {
    /// List of resolved dependencies.
    #[serde(default)]
    pub dependencies: Vec<GradleDependency>,
}

/// A single resolved dependency.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleDependency {
    /// Package scheme (e.g., "pkg").
    pub scheme: Option<String>,
    /// Package type (e.g., "maven").
    #[serde(rename = "type")]
    pub dependency_type: Option<String>,
    /// Package namespace (e.g., Maven group ID).
    pub namespace: Option<String>,
    /// Package name (e.g., Maven artifact ID).
    pub name: Option<String>,
    /// Package version.
    pub version: Option<String>,
    /// Optional key-value qualifiers.
    #[serde(default)]
    pub qualifiers: HashMap<String, String>,
    /// Package URL (purl).
    pub purl: Option<String>,
    /// Repository information.
    pub repository: Option<DependencyRepository>,
}

/// Repository from which a dependency was resolved.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyRepository {
    /// Repository URL.
    pub url: Option<String>,
    /// Repository type.
    #[serde(rename = "type")]
    pub repository_type: Option<String>,
    /// Resolution source (e.g., "localCache", "remote").
    pub resolution_source: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradle_dependencies_deserialize() {
        let json = r#"{
            "dependencies": [
                {
                    "scheme": "pkg",
                    "type": "maven",
                    "namespace": "com.example",
                    "name": "my-library",
                    "version": "1.0.0",
                    "qualifiers": {},
                    "purl": "pkg:maven/com.example/my-library@1.0.0",
                    "repository": {
                        "url": "https://repo.maven.apache.org/maven2/",
                        "type": "maven",
                        "resolutionSource": "remote"
                    }
                }
            ]
        }"#;

        let deps: GradleDependencies = serde_json::from_str(json).unwrap();
        assert_eq!(deps.dependencies.len(), 1);
        let dep = &deps.dependencies[0];
        assert_eq!(dep.dependency_type.as_deref(), Some("maven"));
        assert_eq!(dep.namespace.as_deref(), Some("com.example"));
        assert_eq!(dep.name.as_deref(), Some("my-library"));
        assert_eq!(dep.version.as_deref(), Some("1.0.0"));
        assert_eq!(
            dep.purl.as_deref(),
            Some("pkg:maven/com.example/my-library@1.0.0")
        );
        let repo = dep.repository.as_ref().unwrap();
        assert_eq!(
            repo.url.as_deref(),
            Some("https://repo.maven.apache.org/maven2/")
        );
        assert_eq!(repo.resolution_source.as_deref(), Some("remote"));
    }

    #[test]
    fn test_gradle_dependencies_empty() {
        let json = r#"{"dependencies": []}"#;
        let deps: GradleDependencies = serde_json::from_str(json).unwrap();
        assert!(deps.dependencies.is_empty());
    }

    #[test]
    fn test_gradle_dependency_minimal() {
        let json = r#"{"name": "some-lib"}"#;
        let dep: GradleDependency = serde_json::from_str(json).unwrap();
        assert_eq!(dep.name.as_deref(), Some("some-lib"));
        assert!(dep.namespace.is_none());
        assert!(dep.version.is_none());
        assert!(dep.repository.is_none());
    }
}
