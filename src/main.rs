use clap::{Parser, Subcommand};

mod lib

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    License { license_type: String },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::License { license_type } => println!("Searching for {}...", license_type),
    }
}
