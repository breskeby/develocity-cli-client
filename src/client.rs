//! Async HTTP client for the Develocity API.

use crate::config::IncludeOptions;
use crate::error::{Error, Result};
use crate::models::{Build, GradleAttributes, GradleDeprecations, GradleFailures};
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

    /// Fetch all requested build details in parallel.
    pub async fn get_gradle_build_details(
        &self,
        build_id: &str,
        include: &IncludeOptions,
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
        let (attributes, deprecations, failures) = tokio::join!(
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
            }
        );

        Ok(GradleBuildDetails {
            build,
            attributes: attributes.transpose()?,
            deprecations: deprecations.transpose()?,
            failures: failures.transpose()?,
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
}
