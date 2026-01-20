//! Error types for the Develocity CLI client.

/// All possible errors that can occur in the CLI.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Server URL is not configured.
    #[error("Server URL is required.\n  Set via: --server, DEVELOCITY_SERVER env var, or server in ~/.develocity/config.toml")]
    MissingServer,

    /// Access token is not configured.
    #[error("Access token is required.\n  Set via: --token, DEVELOCITY_ACCESS_KEY env var, or access_key in ~/.develocity/config.toml")]
    MissingToken,

    /// Invalid server URL provided.
    #[error("Invalid server URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Request timed out.
    #[error("Request timed out after {0} seconds")]
    Timeout(u64),

    /// Authentication failed (401).
    #[error("Authentication failed (401).\n  Check your access key or token is valid and not expired.")]
    Unauthorized,

    /// Access denied (403).
    #[error("Access denied (403).\n  Ensure you have the 'Access build data via the API' permission.")]
    Forbidden,

    /// Build not found (404).
    #[error("Build not found: {0}\n  Verify the Build Scan ID is correct.")]
    BuildNotFound(String),

    /// Build is not a Gradle build.
    #[error("Build '{0}' is not a Gradle build (type: {1}).\n  This CLI only supports Gradle builds.")]
    NotGradleBuild(String, String),

    /// Generic API error.
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },

    /// Failed to parse API response.
    #[error("Failed to parse API response: {0}")]
    Parse(String),

    /// Failed to read config file.
    #[error("Failed to read config file '{path}': {source}")]
    ConfigRead {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse config file.
    #[error("Failed to parse config file '{path}': {source}")]
    ConfigParse {
        path: String,
        #[source]
        source: toml::de::Error,
    },
}

impl Error {
    /// Returns the appropriate exit code for this error.
    ///
    /// Exit codes:
    /// - 0: Success (not an error)
    /// - 1: Configuration error
    /// - 2: Network error
    /// - 3: Authentication/authorization error
    /// - 4: Build not found
    /// - 5: Wrong build type (not Gradle)
    /// - 6: API/Parse error
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::MissingServer
            | Error::MissingToken
            | Error::InvalidUrl(_)
            | Error::ConfigRead { .. }
            | Error::ConfigParse { .. } => 1,

            Error::Http(_) | Error::Timeout(_) => 2,

            Error::Unauthorized | Error::Forbidden => 3,

            Error::BuildNotFound(_) => 4,

            Error::NotGradleBuild(_, _) => 5,

            Error::ApiError { .. } | Error::Parse(_) => 6,
        }
    }
}

/// A specialized Result type for CLI operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Exit code constants for documentation and testing.
pub mod exit_codes {
    /// Success - build data retrieved successfully.
    pub const SUCCESS: i32 = 0;
    /// Configuration error - missing server/token, invalid URL, bad config file.
    pub const CONFIG_ERROR: i32 = 1;
    /// Network error - connection failed, timeout.
    pub const NETWORK_ERROR: i32 = 2;
    /// Authentication error - 401 Unauthorized, 403 Forbidden.
    pub const AUTH_ERROR: i32 = 3;
    /// Build not found - Build Scan ID doesn't exist.
    pub const NOT_FOUND: i32 = 4;
    /// Wrong build type - Build is not a Gradle build.
    pub const WRONG_BUILD_TYPE: i32 = 5;
    /// API/Parse error - Unexpected API response.
    pub const API_ERROR: i32 = 6;
}
