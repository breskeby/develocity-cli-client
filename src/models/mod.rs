//! Data models for Develocity API responses.

pub mod build;
pub mod dependencies;
pub mod deprecations;
pub mod failures;
pub mod network_activity;
pub mod task_execution;
pub mod tests;

pub use build::*;
pub use dependencies::*;
pub use deprecations::*;
pub use failures::*;
pub use network_activity::*;
pub use task_execution::*;
pub use tests::*;
