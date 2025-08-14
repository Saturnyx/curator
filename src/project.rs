use crossterm::style::Stylize;
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

        // Download repository as zip file instead of cloning
        let zip_url = "https://github.com/Saturnyx/curator/archive/refs/heads/main.zip";
        let temp_dir = std::env::temp_dir().join("curator_temp");

        // Clean up any existing temp directory
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir).ok();
        }

        match Self::download_and_extract_zip(zip_url, &temp_dir) {
            Ok(_) => {
                println!(
                    "{} Repository downloaded successfully.",
                    "[SUCCESS]".green()
                );

                // Copy the specific template to the current directory
                let template_path = temp_dir
                    .join("curator-main") // GitHub zip extracts with repo-branch format
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
            Err(e) => eprintln!("{} Failed to download repository: {}", "[ERROR]".red(), e),
        }
    }

    fn download_and_extract_zip(
        url: &str,
        extract_to: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Download the zip file
        let response = reqwest::blocking::get(url)?;
        let bytes = response.bytes()?;

        // Create temp directory
        std::fs::create_dir_all(extract_to)?;

        // Extract the zip file
        let cursor = std::io::Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(cursor)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => extract_to.join(path),
                None => continue,
            };

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(())
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
