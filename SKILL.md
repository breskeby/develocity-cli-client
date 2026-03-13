---
name: resolve-gradle-develocity-scan
description: Resolve and analyze Gradle Develocity build scan data from https://gradle-enterprise.example.com using the dvcli command-line tool
license: MIT
compatibility: opencode
metadata:
  audience: developers
  workflow: gradle-develocity
---

## Overview

This skill provides instructions for resolving and analyzing Gradle build scan data from the Elastic Develocity server at `https://gradle-enterprise.example.com`. It uses the `dvcli` command-line tool to fetch build details, deprecations, failures, and test results.

## When to Use

Use this skill when the user:
- Mentions or provides a URL pointing to `https://gradle-enterprise.example.com`
- References a Gradle build scan ID
- Asks about build failures, test results, deprecations, or build performance
- Wants to investigate or debug a specific Gradle build

## Prerequisites

### dvcli Tool

The `dvcli` (Develocity CLI Client) tool must be installed and available in PATH.

Verify installation:
```bash
which dvcli
```

### Environment Variables

The following environment variables should be set for authentication:

- **`DEVELOCITY_SERVER`**: `https://gradle-enterprise.example.com`
- **`DEVELOCITY_API_KEY`**: Your Develocity access token

Verify configuration:
```bash
[ -z "$DEVELOCITY_SERVER" ] && echo "WARNING: DEVELOCITY_SERVER not set" || echo "Server: $DEVELOCITY_SERVER"
[ -z "$DEVELOCITY_API_KEY" ] && echo "WARNING: DEVELOCITY_API_KEY not set" || echo "Access key is configured"
```

### Alternative Configuration

Instead of environment variables, you can create `~/.develocity/config.toml`:

```toml
server = "https://gradle-enterprise.example.com"
token = "your-access-key"
timeout = 60
```

## Extracting Build Scan ID

Build scan URLs follow this pattern:
```
https://gradle-enterprise.example.com/s/{BUILD_SCAN_ID}
```

To extract the build scan ID from a URL:
```bash
# Example URL: https://gradle-enterprise.example.com/s/abc123xyz
BUILD_SCAN_ID=$(echo "https://gradle-enterprise.example.com/s/abc123xyz" | sed 's|.*/s/||')
echo $BUILD_SCAN_ID  # Output: abc123xyz
```

## Usage Patterns

### Pattern 1: Quick Build Summary

Get a human-readable summary of the build including result, deprecations, failures, and tests.

```bash
dvcli build <BUILD_SCAN_ID>
```

**Example**:
```bash
dvcli build abc123xyz
```

**Output includes**:
- Build status (SUCCESS, FAILURE, etc.)
- Build duration
- Project name and Gradle version
- Requested tasks
- Tags and metadata
- Deprecation warnings (if any)
- Build failures (if any)
- Test results summary

---

### Pattern 2: JSON Output for Automation

Get structured JSON output for parsing or automation.

```bash
dvcli build <BUILD_SCAN_ID> -o json
```

**Example**:
```bash
dvcli build abc123xyz -o json | jq '.'
```

**Parse specific fields**:
```bash
# Get build status
dvcli build abc123xyz -o json | jq -r '.result.status'

# Get failure count
dvcli build abc123xyz -o json | jq -r '.tests.summary.failed'

# Get test pass rate
dvcli build abc123xyz -o json | jq -r '.tests.summary.passRate'
```

---

### Pattern 3: Show Only Failures

Get only build and test failures with detailed error messages.

```bash
dvcli build <BUILD_SCAN_ID> -i failures
```

**With verbose stacktraces**:
```bash
dvcli build <BUILD_SCAN_ID> -i failures -v
```

**Example**:
```bash
dvcli build abc123xyz -i failures -v
```

This is useful when investigating build or test failures.

---

### Pattern 4: Show Only Test Results

Get comprehensive test execution results.

```bash
dvcli build <BUILD_SCAN_ID> -i tests
```

**With detailed test output (stdout/stderr, stacktraces)**:
```bash
dvcli build <BUILD_SCAN_ID> -i tests -v
```

**Example**:
```bash
dvcli build abc123xyz -i tests -v
```

**Output includes**:
- Test summary (total, passed, failed, skipped)
- Pass rate
- Duration
- Failed test details with error messages
- With `-v`: full stdout/stderr and stacktraces

---

### Pattern 5: Show Only Build Result

Get only the build result metadata without failures or tests.

```bash
dvcli build <BUILD_SCAN_ID> -i result
```

**Example**:
```bash
dvcli build abc123xyz -i result
```

**Output includes**:
- Build status
- Duration
- Project name
- Gradle version
- Requested tasks
- Tags
- User and hostname

---

### Pattern 6: Show Only Deprecations

Get only deprecation warnings to understand what needs to be upgraded.

```bash
dvcli build <BUILD_SCAN_ID> -i deprecations
```

**Example**:
```bash
dvcli build abc123xyz -i deprecations
```

**Output includes**:
- Deprecated API/feature name
- Removal version (e.g., "Gradle 9.0")
- Migration advice
- Usage count and location

---

### Pattern 7: Custom Combinations

Combine multiple include options (comma-separated):

```bash
# Result + failures (no tests or deprecations)
dvcli build <BUILD_SCAN_ID> -i result,failures

# Result + deprecations (no failures or tests)
dvcli build <BUILD_SCAN_ID> -i result,deprecations

# Failures + tests (no result or deprecations)
dvcli build <BUILD_SCAN_ID> -i failures,tests -v
```

---

### Pattern 8: Direct URL Input

When the user provides a full build scan URL, extract the ID first:

```bash
# Given URL: https://gradle-enterprise.example.com/s/abc123xyz
BUILD_SCAN_ID=$(echo "https://gradle-enterprise.example.com/s/abc123xyz" | sed 's|.*/s/||')
dvcli build $BUILD_SCAN_ID
```

Or use a one-liner:
```bash
dvcli build $(echo "https://gradle-enterprise.example.com/s/abc123xyz" | sed 's|.*/s/||')
```

---

## Include Options Reference

| Option | Description |
|--------|-------------|
| `result` | Build status, duration, project info, Gradle version, tasks, tags |
| `deprecations` | Deprecated API usage with removal versions and migration advice |
| `failures` | Build failures and test failures with error messages |
| `tests` | Complete test execution results (summary + individual test details) |
| `task-execution` | Task cache avoidance, build timing, parallelism, per-task breakdown |
| `network-activity` | HTTP requests, file downloads, timing by method and repository |
| `dependencies` | Resolved dependencies grouped by type (e.g., maven); purl and repository details with `--verbose` |
| `all` | Everything (default if `-i` is not specified) |

**Combine options** with commas: `-i result,failures,tests`

## Verbose Mode

Add `-v` or `--verbose` to show:
- Full stacktraces for failures
- Test stdout/stderr output
- Detailed error contexts
- Per-task details in task execution (cache keys, artifact sizes, types)
- All repositories in network activity (default shows top 5)
- Dependency purl and repository details (default shows top 20 per type)

This is especially useful for:
- Debugging test failures
- Understanding compilation errors
- Investigating runtime exceptions
- Auditing dependency downloads and build cache usage

## Use Case Guide

| User Request | Command Pattern |
|--------------|-----------------|
| "What happened with this build?" | `dvcli build <ID>` |
| "Show me build failures" | `dvcli build <ID> -i failures -v` |
| "What tests failed?" | `dvcli build <ID> -i tests -v` |
| "Show test results" | `dvcli build <ID> -i tests` |
| "What's the build status?" | `dvcli build <ID> -i result` |
| "Any deprecation warnings?" | `dvcli build <ID> -i deprecations` |
| "How did the build cache perform?" | `dvcli build <ID> -i task-execution` |
| "What was downloaded during the build?" | `dvcli build <ID> -i network-activity` |
| "What dependencies does this build use?" | `dvcli build <ID> -i dependencies` |
| "Show full dependency details (purl, repos)" | `dvcli build <ID> -i dependencies -v` |
| "Give me JSON output" | `dvcli build <ID> -o json` |
| "Show failures with stacktraces" | `dvcli build <ID> -i failures -v` |
| "Analyze this build scan: https://..." | Extract ID, then `dvcli build <ID>` |

## Error Handling

### Exit Codes

| Code | Meaning | Action |
|------|---------|--------|
| 0 | Success | Build data retrieved (even if build failed) |
| 1 | Configuration error | Check DEVELOCITY_SERVER and DEVELOCITY_API_KEY |
| 2 | Network error | Check connectivity to gradle-enterprise.example.com |
| 3 | Authentication error | Verify access token is valid |
| 4 | Not found | Build scan ID doesn't exist |
| 5 | Wrong build type | Not a Gradle build (might be Maven, etc.) |
| 6 | API error | Unexpected API response |

### Common Issues

**Configuration Error (exit 1)**:
```
ERROR: DEVELOCITY_SERVER is not set
```
Fix: Set environment variable or create config file.

**Authentication Error (exit 3)**:
```
ERROR: Authentication failed (401 Unauthorized)
```
Fix: Check that DEVELOCITY_API_KEY is valid and has read permissions.

**Not Found (exit 4)**:
```
ERROR: Build scan not found
```
Fix: Verify the build scan ID is correct.

**Wrong Build Type (exit 5)**:
```
ERROR: Build is not a Gradle build
```
This tool only works with Gradle builds. The scan might be for Maven or another build tool.

## API Endpoints Reference

The dvcli tool uses these Develocity API endpoints:

- `GET /api/builds/{id}` - Validate build exists
- `GET /api/builds/{id}/gradle-attributes` - Build result info
- `GET /api/builds/{id}/gradle-deprecations` - Deprecation warnings
- `GET /api/builds/{id}/gradle-failures` - Build and test failures
- `GET /api/tests/build/{id}` - Test execution results
- `GET /api/builds/{id}/gradle-build-cache-performance` - Task execution and cache performance
- `GET /api/builds/{id}/gradle-network-activity` - Network activity
- `GET /api/builds/{id}/gradle-dependencies` - Resolved dependency list

## Examples

### Example 1: Investigate a Failed Build

```bash
# User provides: "This build failed: https://gradle-enterprise.example.com/s/xyz789"

# Extract ID and get failures with details
BUILD_SCAN_ID=$(echo "https://gradle-enterprise.example.com/s/xyz789" | sed 's|.*/s/||')
dvcli build $BUILD_SCAN_ID -i failures -v
```

### Example 2: Check Test Pass Rate

```bash
# Get test summary as JSON
dvcli build abc123 -o json | jq '.tests.summary'

# Output:
# {
#   "total": 150,
#   "passed": 145,
#   "failed": 3,
#   "skipped": 2,
#   "passRate": 96.67
# }
```

### Example 3: Find Deprecation Issues

```bash
# Get all deprecations to plan upgrades
dvcli build abc123 -i deprecations

# Output shows:
# - Which APIs are deprecated
# - When they'll be removed
# - How to migrate
```

### Example 4: Quick Status Check

```bash
# Just get the build status
dvcli build abc123 -o json | jq -r '.result.status'
# Output: SUCCESS or FAILURE
```

## Tips

1. **Use JSON output** (`-o json`) when you need to parse specific fields or integrate with other tools
2. **Use verbose mode** (`-v`) when investigating failures - it provides stacktraces and detailed output
3. **Filter with `-i`** to reduce noise when you only need specific information
4. **Combine with jq** for powerful JSON querying and filtering
5. **Check exit codes** in scripts to handle different error conditions appropriately

## Workflow Integration

This tool is particularly useful for:

- **CI/CD debugging**: Quickly analyze why a build failed without opening the web UI
- **Automated reporting**: Extract test metrics and failure rates programmatically
- **Deprecation tracking**: Monitor deprecated API usage across builds
- **Performance analysis**: Track build durations and identify slow builds
- **Test analytics**: Aggregate test failure patterns across multiple builds
