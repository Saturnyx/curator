use chrono::Datelike;
use chrono::Local;
use crossterm::style::Stylize;
use dialoguer::Input;
use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::sync::Mutex;

pub static CONFIGURATION: Lazy<Mutex<HashMap<String, HashMap<String, String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
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

        // Check if license already exists in configuration
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
                    // Save the validated license to configuration
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
                        input_license.red()
                    );
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
    fn get_licenses() -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
        fs::write(&filename, response)?;
        println!(
            "{}",
            format!("Downloaded {} license to {}", license, filename).green()
        );
        Ok(())
    }

    fn modify_license(mut license: String) -> String {
        // Load config if not already loaded
        ConfigManager::load_config();
        let data_map = {
            let config_guard = CONFIGURATION.lock().unwrap();
            config_guard.get("data").cloned()
        }; // Release lock immediately

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
                                // Save the user input to configuration
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
                            // Create data section and save the user input
                            {
                                let mut config_guard = CONFIGURATION.lock().unwrap();
                                let mut data = HashMap::new();
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
        // Save any new placeholder values that were added
        ConfigManager::save_config();
        license
    }
}

pub struct ConfigManager;
impl ConfigManager {
    /// Loades configuration
    pub fn load_config() {
        if Path::new("curator.json").exists() {
            println!("Found curator.json file");
            let config = Self::get_config();
            let mut guard = CONFIGURATION.lock().unwrap();
            *guard = config;
        } else {
            println!("{}", "Looks like your project isn't configured...".yellow());
            let config = Self::ask_config();
            {
                let mut guard = CONFIGURATION.lock().unwrap();
                *guard = config;
            }
            Self::save_config();
            println!("{} curator.json created", "[SUCCESS]".green());
            println!("{}", "Completed Project Initialization".bold());
            let add_to_gitignore: bool = Input::<String>::new()
                .with_prompt("Do you want to add curator.json to .gitignore? (y/n)")
                .default("y".into())
                .interact_text()
                .unwrap()
                .to_lowercase()
                .starts_with('y');
            Self::gitignored(add_to_gitignore);
        }
    }

    /// Returns hashmap from `curator.json`
    fn get_config() -> HashMap<String, HashMap<String, String>> {
        let content = match fs::read_to_string("curator.json") {
            Ok(c) => c,
            Err(e) => {
                println!(
                    "{} Failed to read curator.json file: {}",
                    "[ERROR]".red(),
                    e
                );
                process::exit(1);
            }
        };
        let map = match serde_json::from_str(&content) {
            Ok(c) => c,
            Err(e) => {
                println!(
                    "{} Failed to parse curator.json file: {}",
                    "[ERROR]".red(),
                    e
                );
                process::exit(1);
            }
        };
        map
    }

    /// Saves config to curator.json from CONFIGURATION
    fn save_config() {
        let config_guard = CONFIGURATION.lock().unwrap();
        match serde_json::to_string_pretty(&*config_guard) {
            Ok(json_string) => {
                if let Err(e) = fs::write("curator.json", json_string) {
                    eprintln!(
                        "{} Failed to write curator.json file: {}",
                        "[ERROR]".red(),
                        e
                    );
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to serialize configuration: {}",
                    "[ERROR]".red(),
                    e
                );
                process::exit(1);
            }
        }
    }

    /// Records repository information from user through prompts
    fn ask_config() -> HashMap<String, HashMap<String, String>> {
        println!("{}", "Initializing Project Configuration:".bold());
        let author: String = Input::new()
            .with_prompt("What is your legal name? ")
            .interact_text()
            .unwrap();
        let project_license: String;
        let licenses = match LicenseManager::get_licenses() {
            Ok(licenses) => licenses,
            Err(e) => {
                eprintln!("{} Could not get Licenses: {}", "[ERROR]".red(), e);
                process::exit(1);
            }
        };
        let licenses_lower: Vec<String> = licenses.iter().map(|l| l.to_lowercase()).collect();
        loop {
            let input_license: String = Input::new()
                .with_prompt("Enter preferred license")
                .interact_text()
                .unwrap();
            let input_license_lower = input_license.to_lowercase();
            let search_name = format!("{}.txt", input_license_lower);
            if let Some(idx) = licenses_lower.iter().position(|l| l == &search_name) {
                project_license = licenses[idx].trim_end_matches(".txt").to_string();
                break;
            } else {
                println!(
                    "{} License '{}' not found in SPDX list. Please try again.",
                    "[ERROR]".red(),
                    input_license.red()
                );
            }
        }
        let year = Local::now().year();
        let project_dir = env::current_dir().unwrap();
        let project_name = project_dir.file_name().unwrap().to_string_lossy();
        let mut map = HashMap::new();
        let mut settings = HashMap::new();
        settings.insert("project".to_string(), project_name.into());
        settings.insert("path".to_string(), project_dir.to_string_lossy().into());
        settings.insert("author".to_string(), author.into());
        let mut data = HashMap::new();
        data.insert("license".to_string(), project_license);
        data.insert("year".to_string(), year.to_string());
        map.insert("data".to_string(), data);
        map.insert("settings".to_string(), settings);
        map
    }
    fn gitignored(in_gitignore: bool) {
        let gitignore_path = Path::new(".gitignore");
        let filename = "curator.json";

        if in_gitignore {
            let mut lines = if gitignore_path.exists() {
                fs::read_to_string(gitignore_path)
                    .unwrap_or_default()
                    .lines()
                    .map(|l| l.trim().to_string())
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };

            if !lines.iter().any(|l| l == filename) {
                lines.push(filename.to_string());
                if let Err(e) = fs::write(gitignore_path, lines.join("\n") + "\n") {
                    eprintln!("{} Failed to update .gitignore: {}", "[ERROR]".red(), e);
                } else {
                    println!("{} Added '{}' to .gitignore", "[SUCCESS]".green(), filename);
                }
            }
        } else {
            // Remove curator.json from .gitignore if present
            if gitignore_path.exists() {
                let lines = fs::read_to_string(gitignore_path)
                    .unwrap_or_default()
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| l != filename && !l.is_empty())
                    .collect::<Vec<_>>();
                if let Err(e) = fs::write(
                    gitignore_path,
                    lines.join("\n")
                        + if lines.is_empty() {
                            ""
                        } else {
                            "\n"
                        },
                ) {
                    eprintln!("{} Failed to update .gitignore: {}", "[ERROR]".red(), e);
                } else {
                    println!(
                        "{} Removed '{}' from .gitignore",
                        "[SUCCESS]".green(),
                        filename
                    );
                }
            }
        }
    }
}
