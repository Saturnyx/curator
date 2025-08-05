use crossterm::style::Stylize;

pub struct Miscellaneous;

impl Miscellaneous {
    pub fn standards() {
        println!("{}", "Community Standards".bold());
        println!(
            "{}",
            if std::path::Path::new("README.md").exists() {
                "README.md".green()
            } else {
                "README.md".red()
            }
        );
        println!(
            "{}",
            if std::path::Path::new("LICENSE").exists() {
                "README.md".green()
            } else {
                "README.md".red()
            }
        );
        println!(
            "{}",
            if std::path::Path::new("CODE_OF_CONDUCT.md").exists() {
                "CODE_OF_CONDUCT.md".green()
            } else {
                "CODE_OF_CONDUCT.md".red()
            }
        );
        println!(
            "{}",
            if std::path::Path::new("CONTRIBUTING.md").exists() {
                "CONTRIBUTING.md".green()
            } else {
                "CONTRIBUTING.md".red()
            }
        );
        println!(
            "{}",
            if std::path::Path::new("SECURITY.md").exists() {
                "SECURITY.md".green()
            } else {
                "SECURITY.md".red()
            }
        );
    }
}
