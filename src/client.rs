//! Async HTTP client for the Develocity API.

use crate::config::IncludeOptions;
use crate::error::{Error, Result};
use crate::models::{Build, GradleAttributes, GradleBuildCachePerformance, GradleDependencies, GradleDeprecations, GradleFailures, GradleNetworkActivity, GradleTests, TestOutcome};
use reqwest::{Client, StatusCode};
use std::time::Duration;
use url::Url;

/// Client for interacting with the Develocity API.
#[derive(Debug, Clone)]
pub struct DevelocityClient {
    http: Client,
    base_url: Url,
    token: String,
}

impl DevelocityClient {
    /// Create a new Develocity client.
    ///
    /// # Arguments
    /// * `server_url` - The base URL of the Develocity server
    /// * `token` - The access key or token for authentication
    /// * `timeout_secs` - Request timeout in seconds
    pub fn new(server_url: &str, token: &str, timeout_secs: u64) -> Result<Self> {
        // Ensure the URL has a trailing slash for proper joining
        let mut url_str = server_url.to_string();
        if !url_str.ends_with('/') {
            url_str.push('/');
        }

        let base_url = Url::parse(&url_str)?;

        let http = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(Error::Http)?;

        Ok(Self {
            http,
            base_url,
            token: token.to_string(),
        })
    }

    /// Get the Build Scan URL for a given build ID.
    pub fn build_scan_url(&self, build_id: &str) -> String {
        format!("{}s/{}", self.base_url, build_id)
    }

    /// Get common build information.
    ///
    /// This is used to validate the build exists and check its type.
    pub async fn get_build(&self, build_id: &str) -> Result<Build> {
        let url = self
            .base_url
            .join(&format!("api/builds/{}", build_id))
            .map_err(Error::InvalidUrl)?;

        self.get_json(url, build_id).await
    }

    /// Get Gradle-specific build attributes.
    pub async fn get_gradle_attributes(&self, build_id: &str) -> Result<GradleAttributes> {
        let url = self
            .base_url
            .join(&format!("api/builds/{}/gradle-attributes", build_id))
            .map_err(Error::InvalidUrl)?;

        self.get_json(url, build_id).await
    }

    /// Get Gradle deprecations.
    pub async fn get_gradle_deprecations(&self, build_id: &str) -> Result<GradleDeprecations> {
        let url = self
            .base_url
            .join(&format!("api/builds/{}/gradle-deprecations", build_id))
            .map_err(Error::InvalidUrl)?;

        self.get_json(url, build_id).await
    }

    /// Get Gradle failures.
    pub async fn get_gradle_failures(&self, build_id: &str) -> Result<GradleFailures> {
        let url = self
            .base_url
            .join(&format!("api/builds/{}/gradle-failures", build_id))
            .map_err(Error::InvalidUrl)?;

        self.get_json(url, build_id).await
    }

    /// Get Gradle network activity.
    ///
    /// Returns `Ok(None)` if network activity is not available (404/403).
    /// Requires Gradle 3.5+ with Develocity Gradle Plugin 1.6+.
    pub async fn get_gradle_network_activity(
        &self,
        build_id: &str,
    ) -> Result<Option<GradleNetworkActivity>> {
        let url = self
            .base_url
            .join(&format!("api/builds/{}/gradle-network-activity", build_id))
            .map_err(Error::InvalidUrl)?;

        match self.get_json_optional(url, build_id).await {
            Ok(activity) => Ok(Some(activity)),
            Err(Error::BuildNotFound(_)) => Ok(None),
            Err(Error::Forbidden) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Get Gradle dependencies.
    ///
    /// Returns `Ok(None)` if dependencies are not available (404/403).
    pub async fn get_gradle_dependencies(
        &self,
        build_id: &str,
    ) -> Result<Option<GradleDependencies>> {
        let url = self
            .base_url
            .join(&format!("api/builds/{}/gradle-dependencies", build_id))
            .map_err(Error::InvalidUrl)?;

        match self.get_json_optional(url, build_id).await {
            Ok(deps) => Ok(Some(deps)),
            Err(Error::BuildNotFound(_)) => Ok(None),
            Err(Error::Forbidden) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Get Gradle build cache performance (task execution timeline).
    pub async fn get_gradle_build_cache_performance(
        &self,
        build_id: &str,
    ) -> Result<GradleBuildCachePerformance> {
        let url = self
            .base_url
            .join(&format!(
                "api/builds/{}/gradle-build-cache-performance",
                build_id
            ))
            .map_err(Error::InvalidUrl)?;

        self.get_json(url, build_id).await
    }

    /// Get Gradle test results.
    ///
    /// Returns `Ok(None)` if tests are not available (404/403).
    /// If `test_outcomes` is non-empty, only tests matching those outcomes are returned
    /// (server-side filtering via `testOutcomes` query parameter).
    pub async fn get_gradle_tests(
        &self,
        build_id: &str,
        test_outcomes: &[TestOutcome],
    ) -> Result<Option<GradleTests>> {
        let mut url = self
            .base_url
            .join(&format!("api/tests/build/{}", build_id))
            .map_err(Error::InvalidUrl)?;

        // Append testOutcomes query params (exploded array: testOutcomes=failed&testOutcomes=flaky)
        if !test_outcomes.is_empty() {
            let mut query_pairs = url.query_pairs_mut();
            for outcome in test_outcomes {
                query_pairs.append_pair("testOutcomes", &outcome.to_string());
            }
            drop(query_pairs);
        }

        match self.get_json_optional(url, build_id).await {
            Ok(tests) => Ok(Some(tests)),
            Err(Error::BuildNotFound(_)) => Ok(None), // No tests available
            Err(Error::Forbidden) => Ok(None),        // Feature not enabled
            Err(e) => Err(e),
        }
    }

    /// Fetch all requested build details in parallel.
    pub async fn get_gradle_build_details(
        &self,
        build_id: &str,
        include: &IncludeOptions,
        test_outcomes: &[TestOutcome],
    ) -> Result<GradleBuildDetails> {
        // First, validate the build exists and is a Gradle build
        let build = self.get_build(build_id).await?;

        if !build.is_gradle() {
            return Err(Error::NotGradleBuild(
                build_id.to_string(),
                build.build_tool_type,
            ));
        }

        // Fetch requested data in parallel
        let (
            attributes,
            deprecations,
            failures,
            tests,
            build_cache_performance,
            network_activity,
            dependencies,
        ) = tokio::join!(
            async {
                if include.result {
                    Some(self.get_gradle_attributes(build_id).await)
                } else {
                    None
                }
            },
            async {
                if include.deprecations {
                    Some(self.get_gradle_deprecations(build_id).await)
                } else {
                    None
                }
            },
            async {
                if include.failures {
                    Some(self.get_gradle_failures(build_id).await)
                } else {
                    None
                }
            },
            async {
                if include.tests {
                    Some(self.get_gradle_tests(build_id, test_outcomes).await)
                } else {
                    None
                }
            },
            async {
                if include.task_execution {
                    Some(self.get_gradle_build_cache_performance(build_id).await)
                } else {
                    None
                }
            },
            async {
                if include.network_activity {
                    Some(self.get_gradle_network_activity(build_id).await)
                } else {
                    None
                }
            },
            async {
                if include.dependencies {
                    Some(self.get_gradle_dependencies(build_id).await)
                } else {
                    None
                }
            }
        );

        // For tests/network_activity/dependencies, we flatten Option<Result<Option<T>>> to Option<T>
        let tests = match tests {
            Some(result) => result?,
            None => None,
        };
        let network_activity = match network_activity {
            Some(result) => result?,
            None => None,
        };
        let dependencies = match dependencies {
            Some(result) => result?,
            None => None,
        };

        Ok(GradleBuildDetails {
            build,
            attributes: attributes.transpose()?,
            deprecations: deprecations.transpose()?,
            failures: failures.transpose()?,
            tests,
            build_cache_performance: build_cache_performance.transpose()?,
            network_activity,
            dependencies,
        })
    }

    /// Make a GET request and deserialize the JSON response.
    async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        url: Url,
        build_id: &str,
    ) -> Result<T> {
        let response = self
            .http
            .get(url)
            .bearer_auth(&self.token)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    Error::Timeout(30) // Default timeout
                } else {
                    Error::Http(e)
                }
            })?;

        let status = response.status();

        match status {
            StatusCode::OK => {
                let text = response.text().await.map_err(Error::Http)?;
                serde_json::from_str(&text).map_err(|e| Error::Parse(e.to_string()))
            }
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::FORBIDDEN => Err(Error::Forbidden),
            StatusCode::NOT_FOUND => Err(Error::BuildNotFound(build_id.to_string())),
            _ => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(Error::ApiError {
                    status: status.as_u16(),
                    message,
                })
            }
        }
    }

    /// Make a GET request for optional data (returns error on 404/403 for caller to handle).
    async fn get_json_optional<T: serde::de::DeserializeOwned>(
        &self,
        url: Url,
        build_id: &str,
    ) -> Result<T> {
        // Reuse the same logic as get_json - the caller handles 404/403 specially
        self.get_json(url, build_id).await
    }
}

/// All requested details for a Gradle build.
#[derive(Debug)]
pub struct GradleBuildDetails {
    /// Common build information.
    pub build: Build,
    /// Gradle-specific attributes (if requested).
    pub attributes: Option<GradleAttributes>,
    /// Deprecations (if requested).
    pub deprecations: Option<GradleDeprecations>,
    /// Failures (if requested).
    pub failures: Option<GradleFailures>,
    /// Test execution results (if requested).
    pub tests: Option<GradleTests>,
    /// Build cache performance / task execution data (if requested).
    pub build_cache_performance: Option<GradleBuildCachePerformance>,
    /// Network activity data (if requested).
    pub network_activity: Option<GradleNetworkActivity>,
    /// Resolved dependencies (if requested).
    pub dependencies: Option<GradleDependencies>,
}
