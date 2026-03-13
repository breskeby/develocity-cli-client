//! Human-readable colored terminal output formatter.

use crate::output::{
    format_duration, BuildFailureOutput, BuildOutput, DeprecationOutput, FailuresOutput,
    NetworkActivityEntryOutput, NetworkActivityOutput, ResultOutput, TaskEntryOutput,
    TaskExecutionOutput, TestExecutionOutput, TestFailureOutput, TestsOutput,
};
use colored::Colorize;

/// Format the build output for human reading in a terminal.
pub fn format(output: &BuildOutput, verbose: bool) -> String {
    let mut lines = Vec::new();

    // Header
    lines.push(format!("Build: {}", output.build_id.bold()));
    lines.push(format!(
        "{} {}",
        "🔗".dimmed(),
        output.build_scan_url.cyan()
    ));
    lines.push("═".repeat(64));
    lines.push(String::new());

    // Result section
    if let Some(ref result) = output.result {
        lines.extend(format_result(result));
        lines.push(String::new());
    }

    // Deprecations section
    if let Some(ref deprecations) = output.deprecations {
        lines.extend(format_deprecations(deprecations));
        lines.push(String::new());
    }

    // Failures section
    if let Some(ref failures) = output.failures {
        lines.extend(format_failures(failures, verbose));
        lines.push(String::new());
    }

    // Tests section
    if let Some(ref tests) = output.tests {
        lines.extend(format_tests(tests, verbose));
        lines.push(String::new());
    }

    // Task execution section
    if let Some(ref task_execution) = output.task_execution {
        lines.extend(format_task_execution(task_execution, verbose));
        lines.push(String::new());
    }

    // Network activity section
    if let Some(ref network_activity) = output.network_activity {
        lines.extend(format_network_activity(network_activity, verbose));
        lines.push(String::new());
    }

    lines.join("\n")
}

fn format_result(result: &ResultOutput) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push(format!("{} Result", "📊".dimmed()));

    // Status with color
    let status = if result.has_failed {
        format!("{} FAILED", "✗".red()).red().bold().to_string()
    } else {
        format!("{} SUCCESS", "✓".green())
            .green()
            .bold()
            .to_string()
    };
    lines.push(format!("   Status:      {}", status));

    // Project name
    if let Some(ref name) = result.project_name {
        lines.push(format!("   Project:     {}", name));
    }

    // Gradle version
    lines.push(format!("   Gradle:      {}", result.gradle_version));

    // Duration
    lines.push(format!(
        "   Duration:    {}",
        format_duration(result.build_duration_ms)
    ));

    // Start time
    let start_time = chrono::DateTime::parse_from_rfc3339(&result.build_start_time)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|_| result.build_start_time.clone());
    lines.push(format!("   Started:     {}", start_time));

    // Tasks
    if !result.requested_tasks.is_empty() {
        lines.push(format!(
            "   Tasks:       {}",
            result.requested_tasks.join(", ")
        ));
    }

    // Tags
    if !result.tags.is_empty() {
        lines.push(format!("   Tags:        {}", result.tags.join(", ")));
    }

    // User and host
    let user_host = match (&result.username, &result.hostname) {
        (Some(user), Some(host)) => Some(format!("{} @ {}", user, host)),
        (Some(user), None) => Some(user.clone()),
        (None, Some(host)) => Some(host.clone()),
        (None, None) => None,
    };
    if let Some(uh) = user_host {
        lines.push(format!("   User:        {}", uh));
    }

    // Failure classification
    if result.has_failed {
        let mut failure_types = Vec::new();
        if result.has_verification_failure == Some(true) {
            failure_types.push("verification".yellow().to_string());
        }
        if result.has_non_verification_failure == Some(true) {
            failure_types.push("non-verification".red().to_string());
        }
        if !failure_types.is_empty() {
            lines.push(format!(
                "   Failure:     {} failures",
                failure_types.join(", ")
            ));
        }
    }

    lines
}

fn format_deprecations(deprecations: &[DeprecationOutput]) -> Vec<String> {
    let mut lines = Vec::new();

    if deprecations.is_empty() {
        lines.push(format!("{} Deprecations (0)", "⚠️ ".dimmed()));
        lines.push(format!("   {}", "No deprecations found".dimmed()));
        return lines;
    }

    lines.push(format!(
        "{} Deprecations ({})",
        "⚠️ ".yellow(),
        deprecations.len().to_string().yellow()
    ));
    lines.push("─".repeat(64));

    for (i, dep) in deprecations.iter().enumerate() {
        lines.push(format!("   {}. {}", i + 1, dep.summary.yellow()));
        lines.push(format!("      Removal:  {}", dep.removal_details.dimmed()));

        if let Some(ref advice) = dep.advice {
            lines.push(format!("      Advice:   {}", advice));
        }

        if let Some(ref url) = dep.documentation_url {
            lines.push(format!("      Doc:      {}", url.cyan()));
        }

        if !dep.usages.is_empty() {
            lines.push("      Used by:".to_string());
            for usage in &dep.usages {
                let location = usage.location.as_deref().unwrap_or("<unknown>");
                lines.push(format!(
                    "        {} {}: {}",
                    "•".dimmed(),
                    usage.owner_type.dimmed(),
                    location
                ));
                if let Some(ref advice) = usage.contextual_advice {
                    lines.push(format!("          {}", advice.dimmed()));
                }
            }
        }

        if i < deprecations.len() - 1 {
            lines.push(String::new());
        }
    }

    lines
}

fn format_failures(failures: &FailuresOutput, verbose: bool) -> Vec<String> {
    let mut lines = Vec::new();

    // Build failures
    if !failures.build_failures.is_empty() {
        lines.push(format!(
            "{} Build Failures ({})",
            "❌".red(),
            failures.build_failures.len().to_string().red()
        ));
        lines.push("─".repeat(64));

        for (i, failure) in failures.build_failures.iter().enumerate() {
            lines.extend(format_build_failure(failure, i + 1, verbose));

            if i < failures.build_failures.len() - 1 {
                lines.push(String::new());
            }
        }

        if !verbose
            && failures
                .build_failures
                .iter()
                .any(|f| f.stacktrace.is_some())
        {
            lines.push(String::new());
            lines.push(format!("   {}", "[Use --verbose for stacktraces]".dimmed()));
        }

        lines.push(String::new());
    }

    // Test failures
    if let Some(ref test_failures) = failures.test_failures {
        if !test_failures.is_empty() {
            lines.push(format!(
                "{} Test Failures ({})",
                "❌".red(),
                test_failures.len().to_string().red()
            ));
            lines.push("─".repeat(64));

            for (i, failure) in test_failures.iter().enumerate() {
                lines.extend(format_test_failure(failure, i + 1, verbose));

                if i < test_failures.len() - 1 {
                    lines.push(String::new());
                }
            }

            if !verbose && test_failures.iter().any(|f| f.stacktrace.is_some()) {
                lines.push(String::new());
                lines.push(format!("   {}", "[Use --verbose for stacktraces]".dimmed()));
            }
        }
    }

    lines
}

fn format_build_failure(failure: &BuildFailureOutput, index: usize, verbose: bool) -> Vec<String> {
    let mut lines = Vec::new();

    // Header (e.g., "Task :compileJava FAILED")
    lines.push(format!("   {}. {}", index, failure.header.red().bold()));

    // Location
    if let Some(ref location) = failure.location {
        lines.push(format!("      Location: {}", location.dimmed()));
    }

    // Message (may be multi-line)
    lines.push(String::new());
    for line in failure.message.lines() {
        lines.push(format!("      {}", line));
    }

    // Stacktrace (only if verbose)
    if verbose {
        if let Some(ref stacktrace) = failure.stacktrace {
            lines.push(String::new());
            lines.push(format!("      {}:", "Stacktrace".dimmed()));
            // Limit stacktrace lines to avoid overwhelming output
            let stack_lines: Vec<&str> = stacktrace.lines().collect();
            let max_lines = 20;
            for line in stack_lines.iter().take(max_lines) {
                lines.push(format!("      {}", line.dimmed()));
            }
            if stack_lines.len() > max_lines {
                lines.push(format!(
                    "      {} more lines...",
                    (stack_lines.len() - max_lines).to_string().dimmed()
                ));
            }
        }
    }

    lines
}

fn format_test_failure(failure: &TestFailureOutput, index: usize, verbose: bool) -> Vec<String> {
    let mut lines = Vec::new();

    // Test name
    let test_name = match &failure.test_name {
        Some(name) => format!("{} > {}", failure.class_name, name),
        None => failure.class_name.clone(),
    };
    lines.push(format!("   {}. {}", index, test_name.red()));

    // Message (may be multi-line, but typically first line is most relevant)
    let first_line = failure.message.lines().next().unwrap_or(&failure.message);
    lines.push(format!("      {}", first_line));

    // Full message if multi-line and verbose
    if verbose {
        let message_lines: Vec<&str> = failure.message.lines().collect();
        if message_lines.len() > 1 {
            for line in message_lines.iter().skip(1) {
                lines.push(format!("      {}", line));
            }
        }

        // Stacktrace
        if let Some(ref stacktrace) = failure.stacktrace {
            lines.push(String::new());
            lines.push(format!("      {}:", "Stacktrace".dimmed()));
            let stack_lines: Vec<&str> = stacktrace.lines().collect();
            let max_lines = 15;
            for line in stack_lines.iter().take(max_lines) {
                lines.push(format!("      {}", line.dimmed()));
            }
            if stack_lines.len() > max_lines {
                lines.push(format!(
                    "      {} more lines...",
                    (stack_lines.len() - max_lines).to_string().dimmed()
                ));
            }
        }
    }

    lines
}

fn format_tests(tests: &TestsOutput, verbose: bool) -> Vec<String> {
    let mut lines = Vec::new();

    // Summary header
    let status_icon = if tests.summary.failed > 0 {
        "❌"
    } else {
        "✓"
    };

    // Build summary parts
    let mut parts = vec![
        format!("{} passed", tests.summary.passed.to_string().green()),
        if tests.summary.failed > 0 {
            format!("{} failed", tests.summary.failed.to_string().red())
        } else {
            format!("{} failed", "0".dimmed())
        },
        format!("{} skipped", tests.summary.skipped.to_string().yellow()),
    ];
    if tests.summary.flaky > 0 {
        parts.push(format!(
            "{} flaky",
            tests.summary.flaky.to_string().yellow()
        ));
    }
    if tests.summary.not_selected > 0 {
        parts.push(format!(
            "{} not selected",
            tests.summary.not_selected.to_string().dimmed()
        ));
    }

    let status_colored = if tests.summary.failed > 0 {
        format!("{} Tests ({})", status_icon, parts.join(", "))
    } else {
        format!("{} Tests ({})", status_icon.green(), parts.join(", "))
    };

    lines.push(status_colored);
    lines.push(format!(
        "   Total: {} tests in {}",
        tests.summary.total,
        format_duration(tests.summary.duration_ms)
    ));
    lines.push(format!("   Pass rate: {:.1}%", tests.summary.pass_rate));
    lines.push("─".repeat(64));

    // Failed tests (always show)
    let failed: Vec<_> = tests
        .tests
        .iter()
        .filter(|t| t.outcome == "failed")
        .collect();

    if !failed.is_empty() {
        lines.push(String::new());
        lines.push(format!("   {} Failed Tests:", "✗".red()));
        for (i, test) in failed.iter().enumerate() {
            lines.extend(format_test_execution(test, i + 1));
        }
    }

    // Flaky tests (always show if present)
    let flaky: Vec<_> = tests
        .tests
        .iter()
        .filter(|t| t.outcome == "flaky")
        .collect();

    if !flaky.is_empty() {
        lines.push(String::new());
        lines.push(format!("   {} Flaky Tests:", "⚠".yellow()));
        for (i, test) in flaky.iter().enumerate() {
            lines.extend(format_test_execution(test, i + 1));
        }
    }

    // Skipped tests (show count, details only if verbose)
    let skipped: Vec<_> = tests
        .tests
        .iter()
        .filter(|t| t.outcome == "skipped")
        .collect();

    if !skipped.is_empty() && verbose {
        lines.push(String::new());
        lines.push(format!("   {} Skipped Tests:", "⊘".yellow()));
        for test in &skipped {
            let name = test
                .test_name
                .as_deref()
                .map(|n| format!("{} > {}", test.class_name, n))
                .unwrap_or_else(|| test.class_name.clone());
            lines.push(format!("      {} {}", "•".dimmed(), name.dimmed()));
        }
    }

    // Passed tests (only show if verbose)
    if verbose {
        let passed: Vec<_> = tests
            .tests
            .iter()
            .filter(|t| t.outcome == "passed")
            .collect();

        if !passed.is_empty() {
            lines.push(String::new());
            lines.push(format!(
                "   {} Passed Tests ({}):",
                "✓".green(),
                passed.len()
            ));
            // Limit to first 20 tests to avoid overwhelming output
            for test in passed.iter().take(20) {
                let name = test
                    .test_name
                    .as_deref()
                    .map(|n| format!("{} > {}", test.class_name, n))
                    .unwrap_or_else(|| test.class_name.clone());
                lines.push(format!(
                    "      {} {} ({})",
                    "✓".green(),
                    name,
                    format_duration(test.duration_ms).dimmed()
                ));
            }
            if passed.len() > 20 {
                lines.push(format!("      ... and {} more", passed.len() - 20));
            }
        }
    }

    lines
}

fn format_test_execution(test: &TestExecutionOutput, index: usize) -> Vec<String> {
    let mut lines = Vec::new();

    let name = test
        .test_name
        .as_deref()
        .map(|n| format!("{} > {}", test.class_name, n))
        .unwrap_or_else(|| test.class_name.clone());

    let outcome_colored = match test.outcome.as_str() {
        "failed" => name.red().to_string(),
        "flaky" => name.yellow().to_string(),
        _ => name.clone(),
    };

    lines.push(format!(
        "      {}. {} ({})",
        index,
        outcome_colored,
        format_duration(test.duration_ms)
    ));

    // Show work unit
    lines.push(format!(
        "         {}",
        format!("Work unit: {}", test.work_unit).dimmed()
    ));

    // Show retry count if there were multiple executions
    if test.execution_count > 1 {
        lines.push(format!(
            "         {}",
            format!("Executions: {} (retried)", test.execution_count).yellow()
        ));
    }

    lines
}

fn format_task_execution(task_exec: &TaskExecutionOutput, verbose: bool) -> Vec<String> {
    let mut lines = Vec::new();

    let summary = &task_exec.summary;
    let savings = &task_exec.avoidance_savings;

    // Header with avoidance ratio
    let ratio_pct = savings.ratio * 100.0;
    let ratio_str = format!("{:.0}%", ratio_pct);
    let ratio_colored = if ratio_pct >= 50.0 {
        ratio_str.green().to_string()
    } else if ratio_pct >= 20.0 {
        ratio_str.yellow().to_string()
    } else {
        ratio_str.to_string()
    };

    lines.push(format!(
        "{} Task Execution ({} tasks, {} avoidance)",
        "⚙️ ".dimmed(),
        summary.total_tasks,
        ratio_colored,
    ));
    lines.push("─".repeat(64));

    // Timing summary
    lines.push(format!(
        "   Build time:       {}",
        format_duration(summary.build_time_ms)
    ));
    lines.push(format!(
        "   Task execution:   {} (effective), {} (serial)",
        format_duration(summary.effective_task_execution_time_ms),
        format_duration(summary.serial_task_execution_time_ms),
    ));
    lines.push(format!(
        "   Parallelism:      {:.1}x serialization factor",
        summary.serialization_factor
    ));

    // Avoidance summary
    lines.push(String::new());
    lines.push(format!(
        "   Avoided:    {} tasks  (saved {})",
        summary.avoided_tasks.to_string().green(),
        format_duration(savings.total_ms).green(),
    ));
    if savings.up_to_date_ms > 0 {
        lines.push(format!(
            "     {} Up-to-date:   {}",
            "•".dimmed(),
            format_duration(savings.up_to_date_ms),
        ));
    }
    if savings.local_build_cache_ms > 0 {
        lines.push(format!(
            "     {} Local cache:  {}",
            "•".dimmed(),
            format_duration(savings.local_build_cache_ms),
        ));
    }
    if savings.remote_build_cache_ms > 0 {
        lines.push(format!(
            "     {} Remote cache: {}",
            "•".dimmed(),
            format_duration(savings.remote_build_cache_ms),
        ));
    }
    lines.push(format!(
        "   Executed:   {} tasks",
        summary.executed_tasks.to_string().yellow(),
    ));

    // Failed tasks (always show)
    let failed: Vec<_> = task_exec.tasks.iter().filter(|t| t.has_failed).collect();
    if !failed.is_empty() {
        lines.push(String::new());
        lines.push(format!("   {} Failed Tasks:", "✗".red()));
        for task in &failed {
            lines.push(format!(
                "      {} {} ({})",
                "✗".red(),
                task.task_path.red(),
                format_duration(task.duration_ms),
            ));
        }
    }

    // Per-task details (verbose only, but show a sample otherwise)
    if verbose {
        lines.push(String::new());
        lines.push(format!("   {} All Tasks:", "▸".dimmed()));
        for task in &task_exec.tasks {
            lines.extend(format_task_entry(task));
        }
    } else if !task_exec.tasks.is_empty() {
        // Show a compact count breakdown by outcome
        let mut outcome_counts: std::collections::BTreeMap<&str, usize> =
            std::collections::BTreeMap::new();
        for task in &task_exec.tasks {
            *outcome_counts.entry(task.outcome.as_str()).or_insert(0) += 1;
        }
        lines.push(String::new());
        lines.push(format!("   {} Breakdown:", "▸".dimmed()));
        for (outcome, count) in &outcome_counts {
            let colored_outcome = match *outcome {
                "UP-TO-DATE" | "FROM-CACHE (local)" | "FROM-CACHE (remote)" => {
                    outcome.green().to_string()
                }
                s if s.starts_with("EXECUTED") => outcome.yellow().to_string(),
                _ => outcome.dimmed().to_string(),
            };
            lines.push(format!("      {:>4}  {}", count, colored_outcome));
        }
        lines.push(String::new());
        lines.push(format!(
            "   {}",
            "[Use --verbose for per-task details]".dimmed()
        ));
    }

    lines
}

fn format_task_entry(task: &TaskEntryOutput) -> Vec<String> {
    let mut lines = Vec::new();

    let outcome_colored = match task.outcome.as_str() {
        "UP-TO-DATE" => task.outcome.green().to_string(),
        s if s.starts_with("FROM-CACHE") => task.outcome.green().to_string(),
        s if s.starts_with("EXECUTED") => task.outcome.yellow().to_string(),
        "NO-SOURCE" | "LIFECYCLE" | "SKIPPED" => task.outcome.dimmed().to_string(),
        _ => task.outcome.clone(),
    };

    let failed_marker = if task.has_failed {
        format!(" {}", "FAILED".red().bold())
    } else {
        String::new()
    };

    lines.push(format!(
        "      {} {} [{}]{}",
        outcome_colored,
        task.task_path,
        format_duration(task.duration_ms).dimmed(),
        failed_marker,
    ));

    // Extra details for verbose mode
    if let Some(ref task_type) = task.task_type {
        // Show short type name (last segment)
        let short_type = task_type.rsplit('.').next().unwrap_or(task_type);
        lines.push(format!("         Type: {}", short_type.dimmed()));
    }
    if let Some(ref reason) = task.non_cacheability_reason {
        lines.push(format!("         Not cacheable: {}", reason.dimmed()));
    }
    if let Some(size) = task.cache_artifact_size {
        lines.push(format!(
            "         Cache artifact: {}",
            format_bytes(size).dimmed()
        ));
    }

    lines
}

fn format_network_activity(activity: &NetworkActivityOutput, verbose: bool) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push(format!(
        "{} Network Activity ({} requests, {})",
        "🌐".dimmed(),
        activity.network_request_count,
        format_bytes(activity.file_download_size_bytes),
    ));
    lines.push("─".repeat(64));

    lines.push(format!(
        "   Requests:     {}",
        activity.network_request_count
    ));
    lines.push(format!(
        "   Downloads:    {} files, {}",
        activity.file_download_count,
        format_bytes(activity.file_download_size_bytes),
    ));
    lines.push(format!(
        "   Time:         {} wall-clock, {} serial",
        format_duration(activity.wall_clock_network_request_time_ms),
        format_duration(activity.serial_network_request_time_ms),
    ));

    // Per-method breakdown
    if !activity.methods.is_empty() {
        lines.push(String::new());
        lines.push(format!("   {} By method:", "▸".dimmed()));
        for entry in &activity.methods {
            lines.extend(format_network_entry(entry));
        }
    }

    // Per-repository breakdown (always show)
    if !activity.repositories.is_empty() {
        lines.push(String::new());
        lines.push(format!("   {} By repository:", "▸".dimmed()));
        let repos_to_show = if verbose {
            activity.repositories.as_slice()
        } else {
            &activity.repositories[..activity.repositories.len().min(5)]
        };
        for entry in repos_to_show {
            lines.extend(format_network_entry(entry));
        }
        if !verbose && activity.repositories.len() > 5 {
            lines.push(format!(
                "   {}",
                format!(
                    "[{} more repositories — use --verbose to see all]",
                    activity.repositories.len() - 5
                )
                .dimmed()
            ));
        }
    }

    lines
}

fn format_network_entry(entry: &NetworkActivityEntryOutput) -> Vec<String> {
    let mut parts = vec![format!(
        "{} reqs",
        entry.network_request_count.to_string().cyan()
    )];
    if entry.file_download_count > 0 {
        parts.push(format!(
            "{} files ({})",
            entry.file_download_count,
            format_bytes(entry.file_download_size_bytes)
        ));
    }
    vec![format!(
        "      {} {}: {}",
        "•".dimmed(),
        entry.name,
        parts.join(", "),
    )]
}

/// Format bytes into a human-readable string.
fn format_bytes(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
