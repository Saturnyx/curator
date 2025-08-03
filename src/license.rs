use crossterm::style::Stylize;
use dialoguer::Input;
use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::process;
use std::sync::Mutex;

use crate::config::{CONFIGURATION, ConfigManager};
use crate::tools::Tools;

static LICENSE_CACHE: Lazy<Mutex<Option<Vec<String>>>> = Lazy::new(|| Mutex::new(None));
static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("curator-app")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client")
});

#[derive(Deserialize)]
struct TreeItem {
    pub path: String,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    pub tree: Vec<TreeItem>,
}

pub struct LicenseManager;

impl LicenseManager {
    
    /// Searches for the license and saves to `LICENSE`
    pub fn set_license(mut license: String) {
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
        license = license.to_lowercase();
        let license_list = match Self::get_licenses() {
            Ok(files) => {
                println!("{} Fetched Licenses", "[SUCCESS]".green());
                files
            }
            Err(e) => {
                eprintln!("{} Could not fetch licenses: {}", "[ERROR]".red(), e);
                process::exit(1);
            }
        };

        let license_list_lower: Vec<String> =
            license_list.iter().map(|l| l.to_lowercase()).collect();

        let selected_license: String;
        let search_name = format!("{}.txt", license);
        if let Some(idx) = license_list_lower.iter().position(|l| l == &search_name) {
            selected_license = license_list[idx].trim_end_matches(".txt").to_string();
            println!(
                "License '{}' found in SPDX list.",
                selected_license.clone().green()
            );
            {
                let mut config_guard = CONFIGURATION.lock().unwrap();
                if let Some(data) = config_guard.get_mut("data") {
                    data.insert("license".to_string(), selected_license.clone());
                }
            }
            ConfigManager::save_config();

            if let Err(e) = Self::download_license(selected_license) {
                eprintln!("{} {}", "[ERROR]".red(), e);
                process::exit(1);
            }
        } else {
            println!(
                "{} License '{}' not found in SPDX list. Please try again.",
                "[ERROR]".red(),
                license.clone().red()
            );
            let similar_licenses = Tools::fuzzy_search(&license_list_lower, &license);
            if !similar_licenses.is_empty() {
                println!("{}", "Did you mean:".yellow());
                for (i, (license, _score)) in similar_licenses.iter().enumerate() {
                    let clean_license = license.trim_end_matches(".txt");
                    println!("  {}. {}", i + 1, clean_license.cyan());
                }
            }
        }
    }

    /// Removes the license
    pub fn remove_license() {
        if let Err(e) = std::fs::remove_file("LICENSE") {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("{} LICENSE file does not exist.", "[INFO]".yellow());
            } else {
                eprintln!("{} Failed to remove LICENSE file: {}", "[ERROR]".red(), e);
                process::exit(1);
            }
        } else {
            println!("{} LICENSE file removed.", "[SUCCESS]".green());
        }
    }

    /// Reloads the license
    pub fn reload_license() {
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
        let license = {
            let config_guard = CONFIGURATION.lock().unwrap();
            config_guard
                .get("data")
                .and_then(|data| data.get("license"))
                .cloned()
        };
        if let Some(license_str) = license {
            if let Err(e) = Self::download_license(license_str) {
                eprintln!("{} {}", "[ERROR]".red(), e);
                process::exit(1);
            }
        } else {
            eprintln!("{} No license configured to edit.", "[ERROR]".red());
            process::exit(1);
        }
    }

    /// Returns a vector of license strings
    pub fn get_licenses() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        {
            let cache_guard = LICENSE_CACHE.lock().unwrap();
            if let Some(ref cached_licenses) = *cache_guard {
                return Ok(cached_licenses.clone());
            }
        }

        let api_url = "https://api.github.com/repos/spdx/license-list-data/git/trees/main:text";
        let response = HTTP_CLIENT.get(api_url).send()?;
        let api_response: ApiResponse = response.json()?;
        let files: Vec<String> = api_response
            .tree
            .into_iter()
            .filter(|item| item.item_type == "blob")
            .map(|item| item.path)
            .collect();

        {
            let mut cache_guard = LICENSE_CACHE.lock().unwrap();
            *cache_guard = Some(files.clone());
        }

        Ok(files)
    }

    /// Downloads License file to `LICENSE`
    fn download_license(license: String) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://raw.githubusercontent.com/spdx/license-list-data/main/text/{}.txt",
            license
        );
        let response = HTTP_CLIENT.get(&url).send()?;

        if !response.status().is_success() {
            return Err(format!(
                "Failed to download license '{}': HTTP {} - {}",
                license,
                response.status().as_u16(),
                response
                    .status()
                    .canonical_reason()
                    .unwrap_or("Unknown error")
            )
            .into());
        }

        let mut response_text = response.text()?;
        response_text = Self::modify_license(response_text);
        let filename = format!("LICENSE");
        std::fs::write(&filename, response_text)?;
        println!(
            "{}",
            format!("Downloaded {} license to {}", license, filename).green()
        );
        Ok(())
    }

    /// Modifies the license either by asking the user or by refering to config
    fn modify_license(mut license: String) -> String {
        ConfigManager::load_config();
        let data_map = {
            let config_guard = CONFIGURATION.lock().unwrap();
            config_guard.get("data").cloned()
        };

        use std::collections::HashMap;
        let mut replacements = HashMap::new();

        let mut start = 0;
        while let Some(open_pos) = license[start..].find('<') {
            let open_pos = start + open_pos;
            if let Some(close_pos) = license[open_pos..].find('>') {
                let close_pos = open_pos + close_pos;
                let placeholder = &license[open_pos + 1..close_pos];

                if !placeholder.starts_with("http") && !replacements.contains_key(placeholder) {
                    let value = if let Some(ref data) = data_map {
                        if let Some(val) = data.get(placeholder) {
                            val.clone()
                        } else {
                            let user_input: String = Input::new()
                                .with_prompt(format!("{}", placeholder))
                                .interact_text()
                                .unwrap();

                            replacements.insert(placeholder.to_string(), user_input.clone());
                            user_input
                        }
                    } else {
                        let user_input: String = Input::new()
                            .with_prompt(format!("{}", placeholder))
                            .interact_text()
                            .unwrap();
                        replacements.insert(placeholder.to_string(), user_input.clone());
                        user_input
                    };
                    replacements.insert(placeholder.to_string(), value);
                }
                start = close_pos + 1;
            } else {
                break;
            }
        }

        for (placeholder, value) in &replacements {
            let pattern = format!("<{}>", placeholder);
            license = license.replace(&pattern, value);
        }

        let mut pos = 0;
        while let Some(start) = license[pos..].find('<') {
            let start = pos + start;
            if let Some(end) = license[start..].find('>') {
                let end = start + end;
                let placeholder = license[start + 1..end].to_string();
                if placeholder.starts_with("http") {
                    license.replace_range(start..=end, &placeholder);
                    pos = start + placeholder.len();
                } else {
                    break; 
                }
            } else {
                break;
            }
        }

        if !replacements.is_empty() {
            let mut config_guard = CONFIGURATION.lock().unwrap();
            if let Some(data) = config_guard.get_mut("data") {
                for (key, value) in replacements {
                    data.insert(key, value);
                }
            } else {
                let mut data = std::collections::HashMap::new();
                for (key, value) in replacements {
                    data.insert(key, value);
                }
                config_guard.insert("data".to_string(), data);
            }
        }

        ConfigManager::save_config();
        license
    }
}
