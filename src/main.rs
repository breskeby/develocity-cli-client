//! Develocity CLI - A command-line client for querying Gradle build information.

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use colored::Colorize;
use dvcli::client::DevelocityClient;
use dvcli::config::{ConfigBuilder, IncludeOptions, OutputFormat};
use dvcli::error::{exit_codes, Error};
use dvcli::output::{self, BuildOutput};
use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

/// Develocity CLI - Query Gradle build information from Develocity
#[derive(Debug, Parser)]
#[command(name = "dvcli")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Get information about a Gradle build
    #[command(alias = "b")]
    Build {
        /// The Build Scan ID (e.g., abc123xyz)
        build_id: String,

        /// Develocity server URL
        #[arg(short, long, env = "DEVELOCITY_SERVER")]
        server: Option<String>,

        /// Access key or token for authentication
        #[arg(short, long, env = "DEVELOCITY_ACCESS_KEY")]
        token: Option<String>,

        /// Output format: json, human
        #[arg(short, long, default_value = "human")]
        output: String,

        /// Data to include: result, deprecations, failures, tests, all
        #[arg(short, long, default_value = "all")]
        include: String,

        /// Show verbose output (stacktraces, etc.)
        #[arg(short, long)]
        verbose: bool,

        /// Request timeout in seconds
        #[arg(long, default_value = "30")]
        timeout: u64,

        /// Config file path
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(cli).await {
        Ok(()) => ExitCode::from(exit_codes::SUCCESS as u8),
        Err(e) => {
            eprintln!("{}: {}", "error".red().bold(), e);
            ExitCode::from(e.exit_code() as u8)
        }
    }
}

async fn run(cli: Cli) -> Result<(), Error> {
    match cli.command {
        Commands::Build {
            build_id,
            server,
            token,
            output,
            include,
            verbose,
            timeout,
            config,
        } => {
            // Parse output format
            let output_format: OutputFormat =
                output.parse().map_err(|e: String| Error::Parse(e))?;

            // Parse include options
            let include_opts = IncludeOptions::parse(&include).map_err(Error::Parse)?;

            // Build configuration
            let cfg = ConfigBuilder::new()
                .server(server)
                .token(token)
                .output_format(Some(output_format))
                .include(Some(include_opts.clone()))
                .verbose(verbose)
                .timeout(Some(timeout))
                .config_file(config)
                .build()?;

            // Create client
            let client = DevelocityClient::new(&cfg.server, &cfg.token, cfg.timeout)?;

            // Fetch build details
            let details = client
                .get_gradle_build_details(&build_id, &cfg.include)
                .await?;

            // Generate output
            let build_scan_url = client.build_scan_url(&build_id);
            let build_output = BuildOutput::from_details(details, build_scan_url, cfg.verbose);

            // Format and print output
            let formatted = match cfg.output_format {
                OutputFormat::Json => output::json::format(&build_output),
                OutputFormat::Human => output::human::format(&build_output, cfg.verbose),
            };

            println!("{}", formatted);

            Ok(())
        }

        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "dvcli", &mut io::stdout());
            Ok(())
        }
    }
}
