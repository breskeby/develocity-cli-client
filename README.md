# dvcli - Develocity CLI Client

A command-line tool for querying Gradle build information from a [Develocity](https://gradle.com/develocity/) server.

## Features

- Query Gradle build scan details from Develocity REST API
- Display build results, deprecations, and failures
- Human-readable colored output or JSON for scripting
- Configurable via CLI arguments, environment variables, or config file
- Shell completions for bash, zsh, fish, and PowerShell

## Installation

### From Source

```bash
git clone https://github.com/your-org/develocity-cli-client.git
cd develocity-cli-client
cargo build --release
# Binary is at target/release/dvcli
```

### Move to PATH (optional)

```bash
cp target/release/dvcli /usr/local/bin/
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
| `-t, --token <TOKEN>` | `DEVELOCITY_ACCESS_KEY` | Access key for authentication | - |
| `-o, --output <FORMAT>` | - | Output format: `json`, `human` | `human` |
| `-i, --include <ITEMS>` | - | Data to include: `result`, `deprecations`, `failures`, `all` | `all` |
| `-v, --verbose` | - | Show stacktraces and verbose output | false |
| `--timeout <SECS>` | - | Request timeout in seconds | `30` |
| `-c, --config <PATH>` | - | Config file path | `~/.develocity/config.toml` |

### Examples

```bash
# Basic usage
dvcli build abc123xyz --server https://ge.example.com --token your-access-key

# Using environment variables
export DEVELOCITY_SERVER=https://ge.example.com
export DEVELOCITY_ACCESS_KEY=your-access-key
dvcli build abc123xyz

# JSON output for scripting
dvcli build abc123xyz -o json | jq '.result.status'

# Only show failures with stacktraces
dvcli build abc123xyz -i failures -v

# Show result and deprecations (no failures)
dvcli build abc123xyz -i result,deprecations
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
  Host:          build-agent-01

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
  "failures": {...}
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
- `GET /api/builds/{id}/gradle-build-cache-performance` - (future)

## Requirements

- Rust 1.70+ (for building from source)
- Develocity server with API access enabled
- Valid access key with read permissions

## License

MIT
