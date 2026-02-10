//! Human-readable colored terminal output formatter.

use crate::output::{
    format_duration, BuildFailureOutput, BuildOutput, DeprecationOutput, FailuresOutput,
    ResultOutput, TestExecutionOutput, TestFailureOutput, TestsOutput,
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

    let status_colored = if tests.summary.failed > 0 {
        format!(
            "{} Tests ({} passed, {} failed, {} skipped)",
            status_icon,
            tests.summary.passed.to_string().green(),
            tests.summary.failed.to_string().red(),
            tests.summary.skipped.to_string().yellow(),
        )
    } else {
        format!(
            "{} Tests ({} passed, {} failed, {} skipped)",
            status_icon.green(),
            tests.summary.passed.to_string().green(),
            "0".dimmed(),
            tests.summary.skipped.to_string().yellow(),
        )
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
            lines.extend(format_test_execution(test, i + 1, verbose));
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

fn format_test_execution(test: &TestExecutionOutput, index: usize, verbose: bool) -> Vec<String> {
    let mut lines = Vec::new();

    let name = test
        .test_name
        .as_deref()
        .map(|n| format!("{} > {}", test.class_name, n))
        .unwrap_or_else(|| test.class_name.clone());

    lines.push(format!(
        "      {}. {} ({})",
        index,
        name.red(),
        format_duration(test.duration_ms)
    ));

    if let Some(ref msg) = test.failure_message {
        let first_line = msg.lines().next().unwrap_or(msg);
        lines.push(format!("         {}", first_line));
    }

    if verbose {
        if let Some(ref stacktrace) = test.stacktrace {
            lines.push(String::new());
            lines.push(format!("         {}:", "Stacktrace".dimmed()));
            for line in stacktrace.lines().take(10) {
                lines.push(format!("         {}", line.dimmed()));
            }
            let total_lines = stacktrace.lines().count();
            if total_lines > 10 {
                lines.push(format!(
                    "         {} more lines...",
                    (total_lines - 10).to_string().dimmed()
                ));
            }
        }

        if let Some(ref stdout) = test.stdout {
            if !stdout.is_empty() {
                lines.push(String::new());
                lines.push(format!("         {}:", "stdout".dimmed()));
                for line in stdout.lines().take(20) {
                    lines.push(format!("         {}", line.dimmed()));
                }
                let total_lines = stdout.lines().count();
                if total_lines > 20 {
                    lines.push(format!(
                        "         {} more lines...",
                        (total_lines - 20).to_string().dimmed()
                    ));
                }
            }
        }

        if let Some(ref stderr) = test.stderr {
            if !stderr.is_empty() {
                lines.push(String::new());
                lines.push(format!("         {}:", "stderr".dimmed()));
                for line in stderr.lines().take(20) {
                    lines.push(format!("         {}", line.dimmed()));
                }
                let total_lines = stderr.lines().count();
                if total_lines > 20 {
                    lines.push(format!(
                        "         {} more lines...",
                        (total_lines - 20).to_string().dimmed()
                    ));
                }
            }
        }
    }

    lines
}
