# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release          # binary: target/release/dvcli

# Test
cargo test
cargo test <test_name>         # run a single test
cargo test -- --nocapture      # show println! output

# Lint
cargo clippy -- -D warnings
cargo fmt --check

# Run
cargo run -- build <BUILD_ID> --server <URL> --token <TOKEN>

# Cross-compile Linux binaries (from macOS, requires `cross`)
./scripts/build-release-binaries.sh
```

## Architecture

`dvcli` is a CLI that fetches Gradle build scan data from a Develocity server and displays it in human-readable or JSON form.

### Data flow

1. **`main.rs`** — parses CLI args (clap), builds a `Config`, dispatches to the `build` subcommand
2. **`config.rs`** — merges config from CLI args > env vars (`DEVELOCITY_SERVER`, `DEVELOCITY_API_KEY`) > `~/.develocity/config.toml` > defaults
3. **`client.rs`** — `DevelocityClient` sends 5 API requests in parallel via `tokio::join!`, validates the build is a Gradle build, returns raw API structs
4. **`output/mod.rs`** — converts raw API structs into `BuildOutput`, applying filters (test outcomes, include flags)
5. **`output/human.rs`** / **`output/json.rs`** — render `BuildOutput` to the terminal or as JSON

### Key design decisions

- **Parallel fetches**: all 5 Develocity API endpoints are fetched concurrently inside `DevelocityClient::fetch_build_data()`
- **`--include` filtering** is applied in `output/mod.rs` before formatting, not in the client
- **`--test-outcomes`** filter is sent as query parameters to the API (server-side filtering)
- **Exit codes 0–6** are defined in `error.rs` and map to distinct failure categories (config, network, auth, not-found, wrong-build-type, API error)

### Module map

| Path | Responsibility |
|------|---------------|
| `src/client.rs` | HTTP client, parallel API fetches |
| `src/config.rs` | Config struct, builder, file loading |
| `src/error.rs` | Error types + exit codes |
| `src/models/` | Serde structs matching Develocity API JSON |
| `src/output/mod.rs` | `BuildOutput` aggregation + 200+ unit tests |
| `src/output/human.rs` | Colored terminal rendering |
| `src/output/json.rs` | JSON rendering |

The bulk of the business logic and test coverage lives in `src/output/mod.rs`.
