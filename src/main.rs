use clap::{Parser, Subcommand};
use curator::*;

#[derive(Parser)]
#[command(name = "curator")]
#[command(about = "A CLI tool for managing project licenses and configuration")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Download and configure a license for your project
    License,
    /// Set project configuration
    Config,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::License => LicenseManager::set_license(),
        Commands::Config => ConfigManager::load_config(),
    }
}
