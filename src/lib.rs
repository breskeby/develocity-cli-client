//! Develocity CLI Client Library
//!
//! This crate provides a Rust client for querying Gradle build information
//! from a Develocity server.
//!
//! # Features
//!
//! - Query build results (success/failure, duration, project info)
//! - Query deprecation warnings
//! - Query build and test failures
//! - Supports both JSON and human-readable output formats
//!
//! # Example
//!
//! ```no_run
//! use dvcli::client::DevelocityClient;
//! use dvcli::config::IncludeOptions;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = DevelocityClient::new(
//!         "https://develocity.example.com",
//!         "your-access-key",
//!         30,
//!     )?;
//!
//!     let details = client
//!         .get_gradle_build_details("abc123xyz", &IncludeOptions::all(), &[])
//!         .await?;
//!
//!     println!("Build failed: {}", details.attributes.map(|a| a.has_failed).unwrap_or(false));
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod output;

pub use client::DevelocityClient;
pub use config::{Config, ConfigBuilder, IncludeOptions, OutputFormat};
pub use error::{Error, Result};
