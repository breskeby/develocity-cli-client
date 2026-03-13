//! Data models for the Gradle Network Activity API response.
//!
//! Maps to `GET /api/builds/{id}/gradle-network-activity`.
//! Requires Gradle 3.5+ with Develocity Gradle Plugin 1.6+.

use serde::Deserialize;
use std::collections::HashMap;

/// Top-level response from `/api/builds/{id}/gradle-network-activity`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradleNetworkActivity {
    /// Total number of network requests.
    pub network_request_count: i64,
    /// Total serial network request time in milliseconds.
    pub serial_network_request_time: i64,
    /// Total wall-clock network request time in milliseconds.
    pub wall_clock_network_request_time: i64,
    /// Total number of file downloads.
    pub file_download_count: i64,
    /// Total file download size in bytes.
    pub file_download_size: i64,
    /// Metrics broken down by HTTP method (e.g., "GET", "PUT").
    #[serde(default)]
    pub methods: HashMap<String, NetworkActivityMetrics>,
    /// Metrics broken down by repository URL.
    #[serde(default)]
    pub repositories: HashMap<String, NetworkActivityMetrics>,
    /// Metrics broken down by HTTP method + repository URL.
    #[serde(default)]
    pub repository_methods: HashMap<String, NetworkActivityMetrics>,
}

/// Network activity metrics for a subset of requests.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkActivityMetrics {
    /// Number of network requests.
    pub network_request_count: i64,
    /// Serial network request time in milliseconds.
    pub serial_network_request_time: i64,
    /// Number of file downloads.
    pub file_download_count: i64,
    /// File download size in bytes.
    pub file_download_size: i64,
    /// Wall-clock network request time in milliseconds.
    pub wall_clock_network_request_time: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradle_network_activity_deserialize() {
        let json = r#"{
            "networkRequestCount": 42,
            "serialNetworkRequestTime": 5000,
            "wallClockNetworkRequestTime": 3000,
            "fileDownloadCount": 10,
            "fileDownloadSize": 1048576,
            "methods": {
                "GET": {
                    "networkRequestCount": 38,
                    "serialNetworkRequestTime": 4800,
                    "fileDownloadCount": 10,
                    "fileDownloadSize": 1048576,
                    "wallClockNetworkRequestTime": 2800
                },
                "HEAD": {
                    "networkRequestCount": 4,
                    "serialNetworkRequestTime": 200,
                    "fileDownloadCount": 0,
                    "fileDownloadSize": 0,
                    "wallClockNetworkRequestTime": 200
                }
            },
            "repositories": {
                "https://repo.example.com/maven2/": {
                    "networkRequestCount": 42,
                    "serialNetworkRequestTime": 5000,
                    "fileDownloadCount": 10,
                    "fileDownloadSize": 1048576,
                    "wallClockNetworkRequestTime": 3000
                }
            },
            "repositoryMethods": {
                "GET https://repo.example.com/maven2/": {
                    "networkRequestCount": 38,
                    "serialNetworkRequestTime": 4800,
                    "fileDownloadCount": 10,
                    "fileDownloadSize": 1048576,
                    "wallClockNetworkRequestTime": 2800
                }
            }
        }"#;

        let activity: GradleNetworkActivity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.network_request_count, 42);
        assert_eq!(activity.serial_network_request_time, 5000);
        assert_eq!(activity.wall_clock_network_request_time, 3000);
        assert_eq!(activity.file_download_count, 10);
        assert_eq!(activity.file_download_size, 1048576);
        assert_eq!(activity.methods.len(), 2);
        assert_eq!(activity.methods["GET"].network_request_count, 38);
        assert_eq!(activity.methods["HEAD"].file_download_count, 0);
        assert_eq!(activity.repositories.len(), 1);
        assert_eq!(activity.repository_methods.len(), 1);
    }

    #[test]
    fn test_network_activity_metrics_deserialize() {
        let json = r#"{
            "networkRequestCount": 5,
            "serialNetworkRequestTime": 1200,
            "fileDownloadCount": 3,
            "fileDownloadSize": 204800,
            "wallClockNetworkRequestTime": 900
        }"#;

        let metrics: NetworkActivityMetrics = serde_json::from_str(json).unwrap();
        assert_eq!(metrics.network_request_count, 5);
        assert_eq!(metrics.serial_network_request_time, 1200);
        assert_eq!(metrics.file_download_count, 3);
        assert_eq!(metrics.file_download_size, 204800);
        assert_eq!(metrics.wall_clock_network_request_time, 900);
    }

    #[test]
    fn test_gradle_network_activity_empty_maps() {
        let json = r#"{
            "networkRequestCount": 0,
            "serialNetworkRequestTime": 0,
            "wallClockNetworkRequestTime": 0,
            "fileDownloadCount": 0,
            "fileDownloadSize": 0,
            "methods": {},
            "repositories": {},
            "repositoryMethods": {}
        }"#;

        let activity: GradleNetworkActivity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.network_request_count, 0);
        assert!(activity.methods.is_empty());
        assert!(activity.repositories.is_empty());
        assert!(activity.repository_methods.is_empty());
    }
}
