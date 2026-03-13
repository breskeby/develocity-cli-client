# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-03-13

### Added

- **Resolved dependencies** (`-i dependencies`): fetches `GET /api/builds/{id}/gradle-dependencies` and displays all resolved dependencies grouped by package type (e.g., maven), sorted by namespace and name. Shows up to 20 per group by default; use `--verbose` to see all entries plus purl and repository details.
- **Network activity** (`-i network-activity`): fetches `GET /api/builds/{id}/gradle-network-activity` and displays total request count, file download size, wall-clock and serial request time, breakdown by HTTP method, and breakdown by repository (top 5 by default; use `--verbose` for all).

## [0.3.0] - 2026-02-24

### Added

- **Task execution timeline** (`-i task-execution`): fetches `GET /api/builds/{id}/gradle-build-cache-performance` and displays build timing, parallelism factor, task avoidance savings (up-to-date, local cache, remote cache), and a per-outcome task count breakdown. Use `--verbose` for full per-task details including cache artifact sizes and non-cacheability reasons.
- **Test outcome filtering** (`--test-outcomes <OUTCOMES>`): filters test results server-side by outcome (comma-separated: `passed`, `failed`, `skipped`, `flaky`, `notSelected`).

## [0.2.1] - 2026-02-18

### Changed

- Published to [crates.io](https://crates.io/crates/dvcli); added `cargo install dvcli` installation instructions.
- Added crates.io metadata to `Cargo.toml` (`description`, `license`, `readme`, `repository`, `homepage`, `documentation`, `keywords`, `categories`).

## [0.2.0] - 2026-02-18

### Added

- **Test execution results** (`-i tests`): fetches `GET /api/tests/build/{id}` and displays a summary (total, passed, failed, skipped, flaky, pass rate, duration) plus per-test details. Failed and flaky tests are always shown; passed tests are shown only with `--verbose` (limited to 20).
- **SKILL.md**: agent skill definition for AI-assisted Develocity build scan analysis.
- Gzip decompression for API responses (reduces transfer size).

### Changed

- Simplified token handling in the HTTP client.

## [0.1.0] - 2026-01-20

### Added

- Initial release.
- `dvcli build <BUILD_SCAN_ID>` command to query Gradle build scan details from a Develocity server.
- Build result summary (`-i result`): status, duration, project name, Gradle version, requested tasks, tags, user, hostname.
- Deprecation warnings (`-i deprecations`): deprecated API/feature name, removal version, migration advice, and usage locations.
- Build and test failures (`-i failures`): error messages with optional stacktraces (`--verbose`).
- Human-readable colored terminal output (default) and JSON output (`-o json`).
- Configuration via CLI arguments, environment variables (`DEVELOCITY_SERVER`, `DEVELOCITY_API_KEY`), or `~/.develocity/config.toml`.
- All requested data sections fetched in parallel via `tokio::join!`.
- Exit codes 0–6 mapping to distinct failure categories (config, network, auth, not-found, wrong-build-type, API error).
- Shell completions for bash, zsh, fish, and PowerShell (`dvcli completions <SHELL>`).

[0.4.0]: https://github.com/breskeby/develocity-cli-client/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/breskeby/develocity-cli-client/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/breskeby/develocity-cli-client/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/breskeby/develocity-cli-client/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/breskeby/develocity-cli-client/releases/tag/v0.1.0
