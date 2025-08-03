use clap::{Parser, Subcommand};
use curator::*;

#[derive(Parser)]
#[command(name = "cu")]
#[command(about = "A CLI tool for managing project licenses and configuration")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Manage project licenses
    License {
        #[command(subcommand)]
        action: LicenseAction,
    },
    /// Set project configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Clone)]
enum LicenseAction {
    /// Set/download a license for your project
    Set {
        /// The name of the license to download and configure
        license_name: String,
    },
    /// Remove the current license file
    Remove,
    /// Reload the current license
    Reload,
}

#[derive(Subcommand, Clone)]
enum ConfigAction {
    /// Set the configuration of the project
    Set,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::License { action } => match action {
            LicenseAction::Set { license_name } => {
                LicenseManager::set_license(license_name);
            }
            LicenseAction::Remove => {
                LicenseManager::remove_license();
            }
            LicenseAction::Reload => {
                LicenseManager::reload_license();
            }
        },
        Commands::Config { action } => match action {
            ConfigAction::Set => {
                ConfigManager::init_config();
            }
        },
    }
}
