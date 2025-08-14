use crossterm::style::Stylize;
use git2::Repository;
use std::io::{self, Write};

pub struct ProjectManager;

impl ProjectManager {
    pub fn init(project_template: String) {
        let local_path = match std::env::current_dir() {
            Ok(path) => match path.into_os_string().into_string() {
                Ok(s) => s,
                Err(_) => {
                    println!("{} Failed to convert path to string.", "[ERROR]".red());
                    std::process::exit(1);
                }
            },
            Err(e) => {
                println!("{} Failed to get current directory: {}", "[ERROR]".red(), e);
                std::process::exit(1);
            }
        };
        if std::fs::read_dir(&local_path)
            .map(|mut i| i.next().is_some())
            .unwrap_or(false)
        {
            println!("{}", "Current directory is not empty.".yellow());
            println!("Do you want to continue? (y/N): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() != "y" {
                std::process::exit(1);
            }
            std::process::exit(1);
        }
        let repo_url: String = format!("https://github.com/Saturnyx/curator/tree/main/templates/project{project_template}");

        match Repository::clone(&repo_url, local_path.clone()) {
            Ok(_) => println!("{} Repository cloned successfully.", "[SUCCESS]".green()),
            Err(e) => eprintln!("{} Failed to clone repository: {}", "[ERROR]".red(), e),
        }
    }
}
