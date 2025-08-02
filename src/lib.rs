use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub mod config;
pub mod license;

pub use config::ConfigManager;
pub use license::LicenseManager;

pub static CONFIGURATION: Lazy<Mutex<HashMap<String, HashMap<String, String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
