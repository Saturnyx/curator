use crossterm::style::Stylize;
use dialoguer::Input;
use once_cell::sync::Lazy;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::process;
use std::sync::Mutex;

use crate::config::{ConfigManager, CONFIGURATION};
use crate::tools::Tools;

static CONDUCT_CACHE: Lazy<Mutex<Option<Vec<String>>>> = Lazy::new(|| Mutex::new(None));
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

pub struct ConductManager;

impl ConductManager {
    /// Searches for the codes of conduct and saves to `CODE_OF_CONDUCT.md`
    pub fn set_conduct(mut license: String) {
        println!("Would you prefer the:");
        println!("  1. Contributor Covenant Code of Conduct - Suitable for projects of all sizes");
        println!("  2. Django Code of Conduct - Suitable for large communities and events");
        let conduct_name = loop {
            let choice: String = Input::new()
            .with_prompt("Enter the number of your choice (1 or 2)")
            .interact_text()
            .unwrap();

            match choice.trim() {
            "1" => break "Contributor Covenant",
            "2" => break "Django",
            _ => {
                println!("{}", "Please enter 1 or 2".red());
                continue;
            }
            }
        };

        println!(
            "You selected the {} Code of Conduct.",
            conduct_name.green()
        );
    }
    
    /// Download the code of conduct
    fn download_conduct(){
        
    }
}
