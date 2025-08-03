use chrono::Datelike;
use chrono::Local;
use crossterm::style::Stylize;
use dialoguer::Input;
use once_cell::sync::Lazy;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::sync::Mutex;

use crate::license::LicenseManager;
use crate::tools::Tools;

pub static CONFIGURATION: Lazy<Mutex<HashMap<String, HashMap<String, String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct ConfigManager;

impl ConfigManager {
    /// Loads configuration
    pub fn load_config() {
        if Self::check_config() {
            let config = Self::get_config();
            let mut guard: std::sync::MutexGuard<'_, HashMap<String, HashMap<String, String>>> =
                CONFIGURATION.lock().unwrap();
            *guard = config;
        } else {
            println!(
                "{} Project configuration not found or has been misconfigured",
                "[ERROR]".red()
            );
            println!(
                "{} Run `{}` to reconfigure project",
                "[FIX]".green(),
                "cu config set".grey()
            );
            process::exit(1);
        }
    }

    /// Asks and creates `curator.json`
    pub fn init_config() {
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
        Self::save_config();
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

    /// Checks if the config is correct
    pub fn check_config() -> bool {
        if !Path::new("curator.json").exists() {
            return false;
        };
        let content = match fs::read_to_string("curator.json") {
            Ok(c) => c,
            Err(_) => {
                return false;
            }
        };
        let content_map = match serde_json::from_str::<HashMap<String, HashMap<String, String>>>(
            &content.as_str(),
        ) {
            Ok(c) => c,
            Err(_) => {
                return false;
            }
        };
        let settings = match content_map.get("settings") {
            Some(c) => c,
            None => {
                return false;
            }
        };
        let path = match settings.get("path") {
            Some(c) => c,
            None => {
                return false;
            }
        };
        if path != &env::current_dir().unwrap().to_string_lossy() {
            return false;
        }
        let project = match settings.get("project") {
            Some(c) => c,
            None => {
                return false;
            }
        };
        if project
            != &env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy()
        {
            return false;
        }
        match settings.get("author") {
            Some(_) => return true,
            None => {
                return false;
            }
        }
    }

    /// Saves config to `curator.json` from CONFIGURATION
    pub fn save_config() {
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
                    input_license.clone().red()
                );
                let similar_licenses = Tools::fuzzy_search(&licenses, &input_license);
                if !similar_licenses.is_empty() {
                    println!("{}", "Did you mean:".yellow());
                    for (i, (license, _score)) in similar_licenses.iter().enumerate() {
                        let clean_license = license.trim_end_matches(".txt");
                        println!("  {}. {}", i + 1, clean_license.cyan());
                    }
                }
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

    // Adds `curator.json` to `.gitignore`
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
            if gitignore_path.exists() {
                let lines = fs::read_to_string(gitignore_path)
                    .unwrap_or_default()
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| l != filename && !l.is_empty())
                    .collect::<Vec<_>>();
                if let Err(e) = fs::write(
                    gitignore_path,
                    lines.join("\n") + if lines.is_empty() { "" } else { "\n" },
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
