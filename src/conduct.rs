use crossterm::style::Stylize;
use dialoguer::Input;
use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use std::process;

use crate::config::{ConfigManager, CONFIGURATION};
use crate::LicenseManager;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("curator-app")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client")
});

pub struct ConductManager;

impl ConductManager {
    /// Searches for the codes of conduct and saves to `CODE_OF_CONDUCT.md`
    pub fn set_conduct() {
        if !ConfigManager::check_config() {
            println!(
                "{} Project configuration not found or has been misconfigured",
                "[ERROR]".red()
            );
            println!(
                "{} Run `{}` to reconfigure project",
                "[FIX]".green(),
                "cu config set".grey()
            );
            process::exit(1)
        } else {
            ConfigManager::load_config();
        }

        println!("Would you prefer the:");
        println!("  1. Contributor Covenant Code of Conduct - Suitable for projects of all sizes");
        println!("  2. Django Code of Conduct - Suitable for large communities and events");
        println!("  3. Citizen Code of Conduct - Suitable for most projects");
        let conduct = loop {
            let choice: String = Input::new()
                .with_prompt("Enter the number of your choice (1, 2 or 3)")
                .interact_text()
                .unwrap();

            match choice.trim() {
                "1" => break "Contributor Covenant",
                "2" => break "Django",
                "3" => break "Citizen",
                _ => {
                    println!("{}", "Please enter 1, 2 or 3".red());
                    continue;
                }
            }
        };

        println!("You selected the {} Code of Conduct.", conduct.green());
        Self::download_conduct(conduct.to_lowercase().replace(" ", "-")).unwrap_or_else(|e| {
            println!(
                "{} Failed to download code of conduct: {}",
                "[ERROR]".red(),
                e
            );
            process::exit(1);
        });

        if conduct == "Citizen" {
            println!("NOTE: Every organization's governing policies should dictate how you handle warnings and expulsions of community members. It is strongly recommended that you mention those policies here or in Section 7 and that you include a mechanism for addressing grievances.");
        }
        {
            let mut config_guard = CONFIGURATION.lock().unwrap();
            if let Some(data) = config_guard.get_mut("data") {
                data.insert(
                    "conduct".to_string(),
                    conduct.to_lowercase().replace(" ", "-"),
                );
            }
        }
        ConfigManager::save_config();
    }

    /// Removes the code of conduct
    pub fn remove_conduct() {
        if let Err(e) = std::fs::remove_file("CODE_OF_CONDUCT.md") {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!(
                    "{} CODE_OF_CONDUCT.md file does not exist.",
                    "[INFO]".yellow()
                );
            } else {
                eprintln!(
                    "{} Failed to remove CODE_OF_CONDUCT.md file: {}",
                    "[ERROR]".red(),
                    e
                );
                process::exit(1);
            }
        } else {
            println!("{} CODE_OF_CONDUCT.md file removed.", "[SUCCESS]".green());
        }
    }

    /// Preview the code of conduct
    pub fn preview_conduct() {
        println!("Would you prefer the:");
        println!("  1. Contributor Covenant Code of Conduct - Suitable for projects of all sizes");
        println!("  2. Django Code of Conduct - Suitable for large communities and events");
        println!("  3. Citizen Code of Conduct - Suitable for most projects");
        let conduct = loop {
            let choice: String = Input::new()
                .with_prompt("Enter the number of your choice (1, 2 or 3)")
                .interact_text()
                .unwrap();

            match choice.trim() {
                "1" => break "Contributor Covenant",
                "2" => break "Django",
                "3" => break "Citizen",
                _ => {
                    println!("{}", "Please enter 1, 2 or 3".red());
                    continue;
                }
            }
        };
        let conduct_formatted = conduct.to_lowercase().replace(" ", "-");
        let url = format!(
            "https://raw.githubusercontent.com/Saturnyx/curator/refs/heads/main/templates/{conduct_formatted}.md"
        );

        let response = match HTTP_CLIENT.get(&url).send() {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("{} Failed to fetch conduct: {}", "[ERROR]".red(), e);
                process::exit(1);
            }
        };

        if !response.status().is_success() {
            println!(
                "Failed to download conduct '{}': HTTP {} - {}",
                conduct,
                response.status().as_u16(),
                response
                    .status()
                    .canonical_reason()
                    .unwrap_or("Unknown error")
            );
            process::exit(1);
        } else {
            println!("{} Loaded Code of Conduct", "SUCCESS".green());
        }
        let response_text = match response.text() {
            Ok(text) => text,
            Err(e) => {
                eprintln!("{} Failed to read license text: {}", "[ERROR]".red(), e);
                process::exit(1);
            }
        };
        println!("{}", "Preview -----".bold());
        println!("{response_text}");
        print!("{}", "End -----".bold())
    }

    /// Download the code of conduct
    fn download_conduct(conduct: String) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://raw.githubusercontent.com/Saturnyx/curator/refs/heads/main/templates/{conduct}.md"
        );
        let response = HTTP_CLIENT.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!(
                "Failed to download '{}': HTTP {} - {}",
                conduct,
                response.status().as_u16(),
                response
                    .status()
                    .canonical_reason()
                    .unwrap_or("Unknown error")
            )
            .into());
        } else {
            println!("{} Loaded Code of Conduct", "SUCCESS".green());
        }

        let mut response_text = response.text()?;
        response_text = LicenseManager::modify_license(response_text);
        let filename = "CODE_OF_CONDUCT.md".to_string();
        std::fs::write(&filename, response_text)?;
        println!("{}", format!("Downloaded {conduct} to {filename}").green());
        Ok(())
    }
}
