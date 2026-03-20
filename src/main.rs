use clap::{Parser, Subcommand};

mod fingerprint;
mod web;
mod database;

#[derive(Parser)]
#[command(name = "secops")]
#[command(about = "ISU SecOps Engine CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pentest modules
    Pentest {
        #[command(subcommand)]
        command: PentestCommands,
    },
    /// Start the integrated Web UI
    Web {
        /// Port to start the server on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}

#[derive(Subcommand)]
enum PentestCommands {
    /// Service fingerprinting on open ports
    Fingerprint {
        /// Target IP or domain
        target: String,

        /// Ports to scan (comma separated, e.g., 22,80,443,3306)
        #[arg(long)]
        ports: String,
 
        /// Number of concurrent scans
        #[arg(long, default_value_t = 100)]
        concurrency: usize,
 
        /// Timeout in seconds for each port scan
        #[arg(long, default_value_t = 3)]
        timeout: u64,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Pentest { command } => match command {
            PentestCommands::Fingerprint { target, ports, concurrency, timeout } => {
                println!("Starting fingerprinting on {} for ports: {} (Concurrency: {}, Timeout: {}s)", target, ports, concurrency, timeout);
                
                let parsed_ports: Result<Vec<u16>, _> = ports.split(',')
                    .map(|p| p.trim().parse::<u16>())
                    .collect();
                
                match parsed_ports {
                    Ok(p) => {
                        if let Err(e) = fingerprint::run_fingerprint(target, p, *concurrency, *timeout).await {
                            eprintln!("Error running fingerprint: {}", e);
                        }
                    }
                    Err(_) => eprintln!("Invalid port list format. use e.g., --ports 22,80,443"),
                }
            }
        },
        Commands::Web { port } => {
            if let Err(e) = web::start_server(*port).await {
                eprintln!("Failed to start web server: {}", e);
            }
        }
    }

    Ok(())
}
