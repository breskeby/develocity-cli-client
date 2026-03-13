# dvcli - Develocity CLI Client

An AI agent friendly command-line tool for querying Gradle build information from a [Develocity](https://gradle.com/develocity/) server.

## Features

- Query Gradle build scan details from Develocity REST API
- Display build results, deprecations, failures, test details, task execution performance, network activity, and resolved dependencies
- Filter test results by outcome (passed, failed, skipped, flaky, notSelected)
- Human-readable colored output or JSON for scripting
- Configurable via CLI arguments, environment variables, or config file
- Shell completions for bash, zsh, fish, and PowerShell

## Installation

### From crates.io

```bash
cargo install dvcli
```

### From Source

```bash
git clone https://github.com/breskeby/develocity-cli-client.git
cd develocity-cli-client
cargo build --release
# Binary is at target/release/dvcli
```

### Move to PATH (optional, source build)

```bash
cp target/release/dvcli /usr/local/bin/
```

### Pre-built binaries (Linux)

See [Releasing](#releasing) to build and publish tarballs. Consumers (e.g. Docker) can then use `DVCLI_RELEASE_BASE_URL` to download instead of building from source.

## Releasing

To build Linux binaries for GitHub Releases (used by the hachi-worker Docker image and others):

1. **From macOS:** install [cross](https://github.com/cross-rs/cross) so you can cross-compile to Linux:
   ```bash
   cargo install cross
   ```

2. **Build and package:**
   ```bash
   ./scripts/build-release-binaries.sh
   ```
   This reads the version from `Cargo.toml`, builds for `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`, and writes:
   - `dist/dvcli-<version>-x86_64-unknown-linux-gnu.tar.gz`
   - `dist/dvcli-<version>-aarch64-unknown-linux-gnu.tar.gz`  
   Each tarball contains a single binary named `dvcli`.

3. **Create a GitHub release and upload the tarballs:**
   ```bash
   gh release create v0.3.0 dist/dvcli-0.3.0-*.tar.gz
   ```

4. **Consumers** can then use the pre-built binary, e.g. in Docker:
   ```bash
   docker build --build-arg DVCLI_RELEASE_BASE_URL=https://github.com/breskeby/develocity-cli-client/releases/download/v0.3.0 -f docker/Dockerfile .
   ```

## Usage

```bash
dvcli build <BUILD_SCAN_ID> [OPTIONS]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `BUILD_SCAN_ID` | The Build Scan ID (e.g., `abc123xyz`) |

### Options

| Option | Environment Variable | Description | Default |
|--------|---------------------|-------------|---------|
| `-s, --server <URL>` | `DEVELOCITY_SERVER` | Develocity server URL | - |
| `-t, --token <TOKEN>` | `DEVELOCITY_API_KEY` | Access key for authentication | - |
| `-o, --output <FORMAT>` | - | Output format: `json`, `human` | `human` |
| `-i, --include <ITEMS>` | - | Data to include: `result`, `deprecations`, `failures`, `tests`, `task-execution`, `network-activity`, `dependencies`, `all` | `all` |
| `-v, --verbose` | - | Show stacktraces, per-task details, and verbose output | false |
| `--test-outcomes <OUTCOMES>` | - | Filter tests by outcome (comma-separated): `passed`, `failed`, `skipped`, `flaky`, `notSelected` | - |
| `--timeout <SECS>` | - | Request timeout in seconds | `30` |
| `-c, --config <PATH>` | - | Config file path | `~/.develocity/config.toml` |

### Examples

```bash
# Basic usage
dvcli build abc123xyz --server https://ge.example.com --token your-access-key

# Using environment variables
export DEVELOCITY_SERVER=https://ge.example.com
export DEVELOCITY_API_KEY=your-access-key
dvcli build abc123xyz

# JSON output for scripting
dvcli build abc123xyz -o json | jq '.result.status'

# Only show failures with stacktraces
dvcli build abc123xyz -i failures -v

# Show result and deprecations (no failures)
dvcli build abc123xyz -i result,deprecations

# Show only test results
dvcli build abc123xyz -i tests

# Show only failed tests
dvcli build abc123xyz -i tests --test-outcomes failed

# Show failed and flaky tests with detailed output
dvcli build abc123xyz -i tests --test-outcomes failed,flaky -v

# Show task execution performance (cache avoidance, per-task timing)
dvcli build abc123xyz -i task-execution

# Show per-task details with cache artifact sizes
dvcli build abc123xyz -i task-execution -v

# Show network activity (downloads, repositories, HTTP methods)
dvcli build abc123xyz -i network-activity

# Show all repositories (not just top 5)
dvcli build abc123xyz -i network-activity -v

# Show resolved dependencies (grouped by type)
dvcli build abc123xyz -i dependencies

# Show dependencies with repository and purl details
dvcli build abc123xyz -i dependencies -v

# Note: the same dependency may appear multiple times in the output if the API
# reports it resolved from multiple sources (e.g., once from local/configuration
# cache without a repository URL, and once from a remote repository).

# Investigate a build failure: result + failures + task execution
dvcli build abc123xyz -i result,failures,task-execution -v
```

## Configuration

### Config File

Create `~/.develocity/config.toml`:

```toml
server = "https://ge.example.com"
token = "your-access-key"
timeout = 60
```

### Configuration Priority

1. CLI arguments (highest priority)
2. Environment variables
3. Config file (`~/.develocity/config.toml`)
4. Default values

## Output

### Human-Readable (default)

```
Build Scan: https://ge.example.com/s/abc123xyz
─────────────────────────────────────────────

Result
  Status:        SUCCESS
  Duration:      45s
  Project:       my-project
  Gradle:        8.5
  Requested:     build
  Tags:          CI, main
  User:          ci-user

Deprecations (2)
  ├─ The BuildListener.buildStarted(Gradle) method has been deprecated
  │  Removal:    Gradle 9.0
  │  Advice:     Use BuildListener.beforeSettings(Settings) instead
  │  Usages:     1 in plugin
  │
  └─ ...

Failures
  Build Failures (1)
    ✗ Execution failed for task ':compileJava'
      Compilation failed; see compiler error output for details.

  Test Failures (3)
    ✗ com.example.MyTest.testSomething
      Expected true but was false

Tests (95 passed, 3 failed, 2 skipped)
  Total: 100 tests in 45s
  Pass rate: 95.0%

  ✗ Failed Tests:
    1. com.example.MyTest > testSomething (120ms)
    2. com.example.OtherTest > testFeature (85ms)

Task Execution (350 tasks, 62% avoidance)
──────────────────────────────────────────
  Build time:       2m 15s
  Task execution:   1m 48s (effective), 12m 30s (serial)
  Parallelism:      6.9x serialization factor

  Avoided:    218 tasks  (saved 8m 45s)
    • Up-to-date:   6m 12s
    • Local cache:  1m 30s
    • Remote cache: 1m 3s
  Executed:   132 tasks

  ▸ Breakdown:
      218  UP-TO-DATE
       45  FROM-CACHE (local)
       12  FROM-CACHE (remote)
       75  EXECUTED (cacheable)

  [Use --verbose for per-task details]
```

### JSON

```bash
dvcli build abc123xyz -o json
```

```json
{
  "build_scan_url": "https://ge.example.com/s/abc123xyz",
  "result": {
    "status": "SUCCESS",
    "duration_millis": 45000,
    "project_name": "my-project",
    "gradle_version": "8.5",
    "requested_tasks": ["build"],
    "tags": ["CI", "main"],
    "user": "ci-user",
    "hostname": "build-agent-01"
  },
  "deprecations": [...],
  "failures": {...},
  "tests": {
    "summary": {
      "total": 100,
      "passed": 95,
      "failed": 3,
      "skipped": 2,
      "durationMs": 45000,
      "passRate": 95.0
    },
    "tests": [...]
  },
  "taskExecution": {
    "summary": {
      "totalTasks": 350,
      "avoidedTasks": 218,
      "executedTasks": 132,
      "buildTimeMs": 135000,
      "serialTaskExecutionTimeMs": 750000,
      "effectiveTaskExecutionTimeMs": 108000,
      "serializationFactor": 6.9
    },
    "avoidanceSavings": {
      "totalMs": 525000,
      "upToDateMs": 372000,
      "localBuildCacheMs": 90000,
      "remoteBuildCacheMs": 63000,
      "ratio": 0.62
    },
    "tasks": [...]
  }
}
```

## Shell Completions

Generate shell completions for your shell:

```bash
# Bash (add to ~/.bashrc)
dvcli completions bash >> ~/.bashrc

# Zsh (add to ~/.zshrc)
dvcli completions zsh >> ~/.zshrc

# Fish
dvcli completions fish > ~/.config/fish/completions/dvcli.fish

# PowerShell (add to profile)
dvcli completions powershell >> $PROFILE
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (even if build failed - failure data is valid output) |
| 1 | Configuration error (missing server/token) |
| 2 | Network error (connection failed, timeout) |
| 3 | Authentication error (invalid token) |
| 4 | Not found (build scan doesn't exist) |
| 5 | Wrong build type (not a Gradle build) |
| 6 | API error (unexpected response) |

## API Endpoints Used

- `GET /api/builds/{id}` - Validate build exists, check build tool type
- `GET /api/builds/{id}/gradle-attributes` - Build result information
- `GET /api/builds/{id}/gradle-deprecations` - Deprecation warnings
- `GET /api/builds/{id}/gradle-failures` - Build and test failures
- `GET /api/tests/build/{id}` - Test execution results
- `GET /api/builds/{id}/gradle-build-cache-performance` - Task execution and cache performance
- `GET /api/builds/{id}/gradle-network-activity` - Network activity (requires Gradle 3.5+ with Develocity Gradle Plugin 1.6+)
- `GET /api/builds/{id}/gradle-dependencies` - Resolved dependency list

## Requirements

- Rust 1.70+ (for building from source)
- Develocity server with API access enabled
- Valid access key with read permissions

## License

MIT
