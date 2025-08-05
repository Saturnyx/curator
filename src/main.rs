use clap::{Parser, Subcommand};
use curator::{conduct::ConductManager, *};

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
    /// Set Code of Conduct
    Conduct {
        #[command(subcommand)]
        action: ConductAction,
    },
    /// Set project configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    Standards,
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
    /// List all licenses
    List,
    /// Preview a license
    Preview {
        /// The name of the license to download and configure
        license_name: String,
    },
}

#[derive(Subcommand, Clone)]
enum ConductAction {
    /// Set/download a Code of Conduct fot your project
    Set,
    /// Remove Code of Conduct
    Remove,
    /// Preview a code of conduct
    Preview,
}

#[derive(Subcommand, Clone)]
enum ConfigAction {
    /// Set the configuration of the project
    Set,
    /// Remove the configuration file
    Remove,
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
            LicenseAction::List => {
                LicenseManager::list_licenses();
            }
            LicenseAction::Preview { license_name } => {
                LicenseManager::preview_license(license_name);
            }
        },
        Commands::Conduct { action } => match action {
            ConductAction::Set => {
                ConductManager::set_conduct();
            }
            ConductAction::Remove => {
                ConductManager::remove_conduct();
            }
            ConductAction::Preview => {
                ConductManager::preview_conduct();
            }
        },
        Commands::Config { action } => match action {
            ConfigAction::Set => {
                ConfigManager::init_config();
            }
            ConfigAction::Remove => {
                ConfigManager::remove_config();
            }
        },
        Commands::Standards => Miscellaneous::standards(),
    }
}
