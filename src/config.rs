//! Configuration loading and merging for the CLI.
//!
//! Configuration is loaded from multiple sources with the following priority:
//! 1. CLI arguments (highest)
//! 2. Environment variables
//! 3. Config file (~/.develocity/config.toml)
//! 4. Built-in defaults (lowest)

use crate::error::{Error, Result};
use crate::models::TestOutcome;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// The default config file path relative to the user's home directory.
const DEFAULT_CONFIG_DIR: &str = ".develocity";
const DEFAULT_CONFIG_FILE: &str = "config.toml";

/// Configuration loaded from the TOML config file.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ConfigFile {
    /// Develocity server URL.
    pub server: Option<String>,
    /// Access token for authentication.
    pub token: Option<String>,
    /// Default output format.
    pub output_format: Option<String>,
    /// Default verbose setting.
    pub verbose: Option<bool>,
    /// Default timeout in seconds.
    pub timeout: Option<u64>,
}

impl ConfigFile {
    /// Load configuration from the default config file path.
    pub fn load_default() -> Result<Self> {
        let path = Self::default_config_path();
        if path.exists() {
            Self::load(&path)
        } else {
            Ok(Self::default())
        }
    }

    /// Load configuration from a specific file path.
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| Error::ConfigRead {
            path: path.display().to_string(),
            source: e,
        })?;

        toml::from_str(&content).map_err(|e| Error::ConfigParse {
            path: path.display().to_string(),
            source: e,
        })
    }

    /// Returns the default config file path (~/.develocity/config.toml).
    pub fn default_config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(DEFAULT_CONFIG_DIR)
            .join(DEFAULT_CONFIG_FILE)
    }
}

/// Resolved configuration after merging all sources.
#[derive(Debug, Clone)]
pub struct Config {
    /// Develocity server URL.
    pub server: String,
    /// Access token for authentication.
    pub token: String,
    /// Output format.
    pub output_format: OutputFormat,
    /// What data to include in the output.
    pub include: IncludeOptions,
    /// Whether to show verbose output (stacktraces, etc.).
    pub verbose: bool,
    /// Request timeout in seconds.
    pub timeout: u64,
    /// Filter test results by these outcomes (server-side). Empty means all.
    pub test_outcomes: Vec<TestOutcome>,
}

/// Output format for the CLI.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OutputFormat {
    /// JSON output for machine consumption.
    Json,
    /// Human-readable colored terminal output.
    #[default]
    Human,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "human" => Ok(OutputFormat::Human),
            other => Err(format!(
                "Invalid output format '{}'. Valid options: json, human",
                other
            )),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Human => write!(f, "human"),
        }
    }
}

/// Options for what data to include in the output.
#[derive(Debug, Clone, Default)]
pub struct IncludeOptions {
    /// Include build result information.
    pub result: bool,
    /// Include deprecation information.
    pub deprecations: bool,
    /// Include failure information.
    pub failures: bool,
    /// Include test execution results.
    pub tests: bool,
    /// Include task execution / build cache performance data.
    pub task_execution: bool,
    /// Include network activity data.
    pub network_activity: bool,
}

impl IncludeOptions {
    /// Create options that include everything.
    pub fn all() -> Self {
        Self {
            result: true,
            deprecations: true,
            failures: true,
            tests: true,
            task_execution: true,
            network_activity: true,
        }
    }

    /// Parse include options from a comma-separated string.
    pub fn parse(s: &str) -> std::result::Result<Self, String> {
        let s = s.trim().to_lowercase();

        if s == "all" {
            return Ok(Self::all());
        }

        let mut opts = Self::default();

        for part in s.split(',') {
            match part.trim() {
                "result" => opts.result = true,
                "deprecations" => opts.deprecations = true,
                "failures" => opts.failures = true,
                "tests" => opts.tests = true,
                "task-execution" => opts.task_execution = true,
                "network-activity" => opts.network_activity = true,
                "all" => return Ok(Self::all()),
                other if !other.is_empty() => {
                    return Err(format!(
                        "Invalid include option '{}'. Valid options: result, deprecations, failures, tests, task-execution, network-activity, all",
                        other
                    ));
                }
                _ => {}
            }
        }

        // If nothing was specified, include everything
        if !opts.result
            && !opts.deprecations
            && !opts.failures
            && !opts.tests
            && !opts.task_execution
            && !opts.network_activity
        {
            return Ok(Self::all());
        }

        Ok(opts)
    }

    /// Returns true if any option is enabled.
    pub fn any(&self) -> bool {
        self.result
            || self.deprecations
            || self.failures
            || self.tests
            || self.task_execution
            || self.network_activity
    }
}

/// Builder for creating a resolved Config from multiple sources.
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    server: Option<String>,
    token: Option<String>,
    output_format: Option<OutputFormat>,
    include: Option<IncludeOptions>,
    verbose: Option<bool>,
    timeout: Option<u64>,
    config_file_path: Option<PathBuf>,
    test_outcomes: Vec<TestOutcome>,
}

impl ConfigBuilder {
    /// Create a new config builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the server URL (from CLI arg).
    pub fn server(mut self, server: Option<String>) -> Self {
        if server.is_some() {
            self.server = server;
        }
        self
    }

    /// Set the access token (from CLI arg).
    pub fn token(mut self, token: Option<String>) -> Self {
        if token.is_some() {
            self.token = token;
        }
        self
    }

    /// Set the output format (from CLI arg).
    pub fn output_format(mut self, format: Option<OutputFormat>) -> Self {
        if format.is_some() {
            self.output_format = format;
        }
        self
    }

    /// Set the include options (from CLI arg).
    pub fn include(mut self, include: Option<IncludeOptions>) -> Self {
        if include.is_some() {
            self.include = include;
        }
        self
    }

    /// Set verbose mode (from CLI arg).
    pub fn verbose(mut self, verbose: bool) -> Self {
        if verbose {
            self.verbose = Some(true);
        }
        self
    }

    /// Set timeout (from CLI arg).
    pub fn timeout(mut self, timeout: Option<u64>) -> Self {
        if timeout.is_some() {
            self.timeout = timeout;
        }
        self
    }

    /// Set the config file path.
    pub fn config_file(mut self, path: Option<PathBuf>) -> Self {
        self.config_file_path = path;
        self
    }

    /// Set the test outcome filters.
    pub fn test_outcomes(mut self, outcomes: Vec<TestOutcome>) -> Self {
        self.test_outcomes = outcomes;
        self
    }

    /// Build the resolved configuration by merging all sources.
    pub fn build(self) -> Result<Config> {
        // Load config file
        let config_file = match &self.config_file_path {
            Some(path) => ConfigFile::load(path)?,
            None => ConfigFile::load_default()?,
        };

        // Resolve server: CLI > env > config file
        let server = self
            .server
            .or_else(|| std::env::var("DEVELOCITY_SERVER").ok())
            .or(config_file.server)
            .ok_or(Error::MissingServer)?;

        // Resolve token: CLI > env > config file
        let token = self
            .token
            .or_else(|| std::env::var("DEVELOCITY_API_KEY").ok())
            .or(config_file.token)
            .ok_or(Error::MissingToken)?;

        // Resolve output format: CLI > config file > default
        let output_format = self.output_format.unwrap_or_else(|| {
            config_file
                .output_format
                .and_then(|s| s.parse().ok())
                .unwrap_or_default()
        });

        // Resolve include options: CLI > default (all)
        let include = self.include.unwrap_or_else(IncludeOptions::all);

        // Resolve verbose: CLI > config file > default (false)
        let verbose = self
            .verbose
            .unwrap_or_else(|| config_file.verbose.unwrap_or(false));

        // Resolve timeout: CLI > config file > default (30)
        let timeout = self
            .timeout
            .unwrap_or_else(|| config_file.timeout.unwrap_or(30));

        Ok(Config {
            server,
            token,
            output_format,
            include,
            verbose,
            timeout,
            test_outcomes: self.test_outcomes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_include_options_parse_all() {
        let opts = IncludeOptions::parse("all").unwrap();
        assert!(opts.result);
        assert!(opts.deprecations);
        assert!(opts.failures);
        assert!(opts.tests);
        assert!(opts.task_execution);
        assert!(opts.network_activity);
    }

    #[test]
    fn test_include_options_parse_single() {
        let opts = IncludeOptions::parse("result").unwrap();
        assert!(opts.result);
        assert!(!opts.deprecations);
        assert!(!opts.failures);
        assert!(!opts.tests);
        assert!(!opts.task_execution);
    }

    #[test]
    fn test_include_options_parse_multiple() {
        let opts = IncludeOptions::parse("result,failures").unwrap();
        assert!(opts.result);
        assert!(!opts.deprecations);
        assert!(opts.failures);
        assert!(!opts.tests);
        assert!(!opts.task_execution);
    }

    #[test]
    fn test_include_options_parse_tests() {
        let opts = IncludeOptions::parse("tests").unwrap();
        assert!(!opts.result);
        assert!(!opts.deprecations);
        assert!(!opts.failures);
        assert!(opts.tests);
        assert!(!opts.task_execution);
    }

    #[test]
    fn test_include_options_parse_multiple_with_tests() {
        let opts = IncludeOptions::parse("result,tests").unwrap();
        assert!(opts.result);
        assert!(!opts.deprecations);
        assert!(!opts.failures);
        assert!(opts.tests);
        assert!(!opts.task_execution);
    }

    #[test]
    fn test_include_options_parse_task_execution() {
        let opts = IncludeOptions::parse("task-execution").unwrap();
        assert!(!opts.result);
        assert!(!opts.deprecations);
        assert!(!opts.failures);
        assert!(!opts.tests);
        assert!(opts.task_execution);
    }

    #[test]
    fn test_include_options_parse_network_activity() {
        let opts = IncludeOptions::parse("network-activity").unwrap();
        assert!(!opts.result);
        assert!(!opts.deprecations);
        assert!(!opts.failures);
        assert!(!opts.tests);
        assert!(!opts.task_execution);
        assert!(opts.network_activity);
    }

    #[test]
    fn test_include_options_parse_empty() {
        let opts = IncludeOptions::parse("").unwrap();
        // Empty means all
        assert!(opts.result);
        assert!(opts.deprecations);
        assert!(opts.failures);
        assert!(opts.tests);
        assert!(opts.task_execution);
        assert!(opts.network_activity);
    }

    #[test]
    fn test_include_options_parse_invalid() {
        let result = IncludeOptions::parse("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_output_format_parse() {
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("JSON".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!(
            "human".parse::<OutputFormat>().unwrap(),
            OutputFormat::Human
        );
        assert!("invalid".parse::<OutputFormat>().is_err());
    }
}
