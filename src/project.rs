use crossterm::style::Stylize;
use git2::Repository;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

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
        }
        // Clone the entire repository first, then copy the specific template
        let repo_url = "https://github.com/Saturnyx/curator.git";
        let temp_dir = std::env::temp_dir().join("curator_temp");

        // Clean up any existing temp directory
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir).ok();
        }

        match Repository::clone(repo_url, &temp_dir) {
            Ok(_) => {
                println!("{} Repository cloned successfully.", "[SUCCESS]".green());

                // Copy the specific template to the current directory
                let template_path = temp_dir
                    .join("templates")
                    .join("project")
                    .join(&project_template);

                if template_path.exists() {
                    match Self::copy_dir(&template_path, Path::new(&local_path)) {
                        Ok(_) => println!("{} Template copied successfully.", "[SUCCESS]".green()),
                        Err(e) => eprintln!("{} Failed to copy template: {}", "[ERROR]".red(), e),
                    }
                } else {
                    eprintln!(
                        "{} Template '{}' not found in repository.",
                        "[ERROR]".red(),
                        project_template
                    );
                }

                // Clean up temp directory
                std::fs::remove_dir_all(&temp_dir).ok();
            }
            Err(e) => eprintln!("{} Failed to clone repository: {}", "[ERROR]".red(), e),
        }
    }
    fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                Self::copy_dir(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }
}
