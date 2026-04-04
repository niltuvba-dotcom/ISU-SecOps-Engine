use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, Table};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tokio::sync::mpsc;

mod database;
mod fingerprint;
mod web;

/// The main command line interface structure for the Aetheris Engine.
#[derive(Parser)]
#[command(name = "aetheris")]
#[command(version = "1.4.0")]
#[command(
    about = "🛡️ Aetheris Engine - Professional Service Fingerprinting & Recon Engine",
    long_about = None
)]
struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    command: Commands,
}

/// Available subcommands for the Aetheris Engine.
#[derive(Subcommand)]
enum Commands {
    /// 🕵️ Pentest & Service Fingerprinting
    Pentest {
        /// Target IP, Hostname or CIDR (e.g. 192.168.1.0/24)
        target: String,

        /// Ports to scan (comma separated, e.g., 22,80,443)
        #[arg(short, long, default_value = "80,443,22,21,3306,5432")]
        ports: String,

        /// Number of concurrent scans
        #[arg(short, long, default_value_t = 100)]
        concurrency: usize,

        /// Timeout in seconds for each port scan
        #[arg(short, long, default_value_t = 3)]
        timeout: u64,

        /// Output format for results
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Table)]
        output: OutputFormat,
    },
    /// 🌐 Start the integrated Web UI
    Web {
        /// Port to start the server on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}

/// Supported output formats for terminal results.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// Human-readable UTF-8 table.
    Table,
    /// JSON for machine processing.
    Json,
    /// Comma Separated Values.
    Csv,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Pentest {
            target,
            ports,
            concurrency,
            timeout,
            output,
        } => {
            run_cli_pentest(target, ports, *concurrency, *timeout, *output).await?;
        }
        Commands::Web { port } => {
            println!("{}", "🌐 Starting Aetheris Web Dashboard...".cyan().bold());
            println!(
                "📍 Local address: {}",
                format!("http://127.0.0.1:{}", port)
                    .bright_white()
                    .underline()
            );
            if let Err(e) = web::start_server(*port).await {
                eprintln!("{} {}", "❌ Error:".red().bold(), e);
            }
        }
    }

    Ok(())
}

/// Orchestrates the pentest operation for a given target and set of ports from the CLI.
/// 
/// It displays a progress bar and renders the final results in the requested format.
async fn run_cli_pentest(
    target: &str,
    ports_str: &str,
    concurrency: usize,
    timeout: u64,
    output: OutputFormat,
) -> anyhow::Result<()> {
    let parsed_ports: Vec<u16> = ports_str
        .split(',')
        .filter_map(|p| p.trim().parse::<u16>().ok())
        .collect();

    if parsed_ports.is_empty() {
        anyhow::bail!("Invalid port list format. Use e.g., --ports 22,80,443");
    }

    if output == OutputFormat::Table {
        println!("{}", "\n🛡️ Aetheris Security Scan".cyan().bold());
        println!("{} {}\n", "Target:".bright_white(), target.yellow());
    }

    let (tx, mut rx) = mpsc::unbounded_channel();

    // Expansion for progress bar estimation
    let targets = fingerprint::expand_target(target);
    let total_tasks = targets.len() * parsed_ports.len();

    let pb = if output == OutputFormat::Table {
        let p = ProgressBar::new(total_tasks as u64);
        p.set_style(
            ProgressStyle::with_template(
                "{spinner:.magenta} [{elapsed_precise}] [{bar:40.magenta/cyan}] {pos}/{len} ({eta}) {msg}",
            )
            .unwrap()
            .progress_chars("━╾─"),
        );
        p.enable_steady_tick(Duration::from_millis(100));
        Some(p)
    } else {
        None
    };

    let t = target.to_string();
    let p_clone = parsed_ports.clone();
    tokio::spawn(async move {
        let _ =
            fingerprint::run_fingerprint_streaming(&t, p_clone, concurrency, timeout, tx).await;
    });

    let mut results = vec![];
    while let Some(res) = rx.recv().await {
        results.push(res);
        if let Some(ref p) = pb {
            p.inc(1);
        }
    }

    if let Some(p) = pb {
        p.finish_with_message("Scan complete!");
    }

    // Handle outputs
    match output {
        OutputFormat::Table => {
            if results.is_empty() {
                println!("\n{}", "No open ports found or target is down.".yellow());
            } else {
                let mut table = Table::new();
                table.load_preset(UTF8_FULL).set_header(vec![
                    Cell::new("TARGET")
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Cyan),
                    Cell::new("PORT")
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Cyan),
                    Cell::new("SERVICE")
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Cyan),
                    Cell::new("VERSION & INTELLIGENCE")
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Cyan),
                ]);

                for res in results {
                    let version_text = if res.version != "unknown" {
                        format!(
                            "{} {}",
                            res.version,
                            format!(
                                "(CVE: google.com/search?q={}+{})",
                                res.service, res.version
                            )
                            .dimmed()
                        )
                    } else {
                        "unknown".to_string()
                    };

                    table.add_row(vec![
                        Cell::new(res.target),
                        Cell::new(format!("{}/tcp", res.port)).fg(Color::Green),
                        Cell::new(res.service).add_attribute(Attribute::Bold),
                        Cell::new(version_text),
                    ]);
                }
                println!("\n{}", table);
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
        OutputFormat::Csv => {
            println!("target,port,state,service,version");
            for r in results {
                println!(
                    "{},{},{},{},{}",
                    r.target, r.port, r.state, r.service, r.version
                );
            }
        }
    }

    Ok(())
}
