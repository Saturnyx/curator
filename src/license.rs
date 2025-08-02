use crossterm::style::Stylize;
use dialoguer::Input;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::process;

use crate::config::{ConfigManager, CONFIGURATION};
use crate::tools::Tools;

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
    pub fn set_license() {
        ConfigManager::load_config();

        let existing_license = {
            let config_guard = CONFIGURATION.lock().unwrap();
            config_guard
                .get("data")
                .and_then(|data| data.get("license"))
                .cloned()
        };

        let project_license = if let Some(license) = existing_license {
            println!("Using configured license: {}", license.clone().green());
            license
        } else {
            let all_licenses = match Self::get_licenses() {
                Ok(files) => {
                    println!("{} Fetched Licenses", "[SUCCESS]".green());
                    files
                }
                Err(e) => {
                    eprintln!("{} Could not fetch licenses: {}", "[ERROR]".red(), e);
                    process::exit(1);
                }
            };

            let all_licenses_lower: Vec<String> =
                all_licenses.iter().map(|l| l.to_lowercase()).collect();

            let selected_license: String;
            loop {
                let input_license: String = Input::new()
                    .with_prompt("Enter preferred license")
                    .interact_text()
                    .unwrap();
                let input_license_lower = input_license.to_lowercase();
                let search_name = format!("{}.txt", input_license_lower);
                if let Some(idx) = all_licenses_lower.iter().position(|l| l == &search_name) {
                    selected_license = all_licenses[idx].trim_end_matches(".txt").to_string();
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
                    break;
                } else {
                    println!(
                        "{} License '{}' not found in SPDX list. Please try again.",
                        "[ERROR]".red(),
                        input_license.clone().red()
                    );
                    
                    // Show top 3 similar licenses using fuzzy search
                    let similar_licenses = Tools::fuzzy_search(&all_licenses, &input_license);
                    if !similar_licenses.is_empty() {
                        println!("{}", "Did you mean:".yellow());
                        for (i, (license, _score)) in similar_licenses.iter().enumerate() {
                            let clean_license = license.trim_end_matches(".txt");
                            println!("  {}. {}", i + 1, clean_license.cyan());
                        }
                    }
                }
            }
            selected_license
        };

        if let Err(e) = Self::download_license(project_license) {
            eprintln!("{} {}", "[ERROR]".red(), e);
            process::exit(1);
        }
    }

    /// Returns a vector of license strings
    pub fn get_licenses() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let api_url = "https://api.github.com/repos/spdx/license-list-data/git/trees/main:text";
        let client = Client::new();
        let response = client
            .get(api_url)
            .header("User-Agent", "curator-app")
            .send()?;
        let api_response: ApiResponse = response.json()?;
        let files = api_response
            .tree
            .into_iter()
            .filter(|item| item.item_type == "blob")
            .map(|item| item.path)
            .collect();
        Ok(files)
    }

    /// Downloads License file to `LICENSE`
    fn download_license(license: String) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://raw.githubusercontent.com/spdx/license-list-data/main/text/{}.txt",
            license
        );
        let client = Client::new();
        let mut response = client
            .get(&url)
            .header("User-Agent", "curator-app")
            .send()?
            .text()?;
        response = Self::modify_license(response);
        let filename = format!("LICENSE");
        std::fs::write(&filename, response)?;
        println!(
            "{}",
            format!("Downloaded {} license to {}", license, filename).green()
        );
        Ok(())
    }

    fn modify_license(mut license: String) -> String {
        ConfigManager::load_config();
        let data_map = {
            let config_guard = CONFIGURATION.lock().unwrap();
            config_guard.get("data").cloned()
        };

        while license.contains('<') || license.contains('>') {
            if let Some(start) = license.find('<') {
                if let Some(end) = license[start..].find('>') {
                    let end = start + end;
                    let placeholder = &license[start + 1..end];
                    if placeholder.starts_with("http") {
                        let link = placeholder.to_string();
                        license.replace_range(start..=end, &link);
                    } else {
                        let value = if let Some(ref data) = data_map {
                            if let Some(val) = data.get(placeholder) {
                                val.clone()
                            } else {
                                let user_input: String = Input::new()
                                    .with_prompt(format!("{}", placeholder))
                                    .interact_text()
                                    .unwrap();
                                {
                                    let mut config_guard = CONFIGURATION.lock().unwrap();
                                    if let Some(data) = config_guard.get_mut("data") {
                                        data.insert(placeholder.to_string(), user_input.clone());
                                    }
                                }
                                user_input
                            }
                        } else {
                            let user_input: String = Input::new()
                                .with_prompt(format!("{}", placeholder))
                                .interact_text()
                                .unwrap();
                            {
                                let mut config_guard = CONFIGURATION.lock().unwrap();
                                let mut data = std::collections::HashMap::new();
                                data.insert(placeholder.to_string(), user_input.clone());
                                config_guard.insert("data".to_string(), data);
                            }
                            user_input
                        };
                        license.replace_range(start..=end, &value);
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        ConfigManager::save_config();
        license
    }
}
