# Curator

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

A powerful command-line tool for managing project licenses and configuration with smart license detection and interactive setup.

[![Get it from the Snap Store](https://snapcraft.io/en/dark/install.svg)](https://snapcraft.io/curator-cli)

## ✨ Features

- 🔍 **Smart License Management**: Automatically fetch and download licenses from the SPDX License List
- 🎯 **Fuzzy Search**: Intelligent license name matching with suggestions for typos
- ⚙️ **Project Configuration**: Interactive project setup with persistent configuration
- 🔄 **License Reloading**: Easily update or reload existing licenses
- 📝 **Template Processing**: Automatic placeholder replacement in license templates
- 🗂️ **Git Integration**: Optional .gitignore management for configuration files

## 🚀 Installation

### From Source

```bash
git clone https://github.com/Saturnyx/curator.git
cd curator
cargo build --release
```

The binary will be available at `target/release/curator` (or `curator.exe` on Windows).

### Using Cargo

Method 1: Install directly from crates.io

```bash
cargo install curator_cli
```

Method 2: Install from the repository

```bash
git clone https://github.com/Saturnyx/curator.git
cd curator
cargo install --path .
```

### Snap Package

Curator is available as a Snap package for easy installation on Linux:

```bash
sudo snap install curator-cli
sudo snap alias curator-cli.curator cu
```

## 📖 Usage

Curator uses the command alias `cu` for convenience.

### Initial Setup

Before using license management features, initialize your project configuration:

```bash
cu config set
```

This will prompt you for:

- Your legal name (for copyright notices)
- Preferred license type
- Whether to add `curator.json` to `.gitignore`

### License Management

#### Set a License

Download and configure a license for your project:

```bash
cu license set MIT
cu license set apache-2.0
cu license set GPL-3.0
```

If you make a typo, Curator will suggest similar license names:

```bash
cu license set ap
# License 'ap' not found in SPDX list. Please try again.
# Did you mean:
#   1. Apache-2.0
#   2. APSL-2.0
#   3. APL-1.0
```

#### Remove License

Remove the current LICENSE file:

```bash
cu license remove
```

#### Reload License

Reload the license from your configuration (useful after updating project details):

```bash
cu license reload
```

### Project Configuration Management

#### View Current Configuration

The project configuration is stored in `curator.json`:

```json
{
  "data": {
    "year": "2025",
    "license": "MIT",
    "copyright holders": "Your Name"
  },
  "settings": {
    "path": "/path/to/your/project",
    "author": "Your Name",
    "project": "project-name"
  }
}
```

#### Reconfigure Project

To update your project configuration:

```bash
cu config set
```

## 🛠️ How It Works

### License Processing

1. **Fetches Available Licenses**: Curator connects to the SPDX License List GitHub repository to get the latest available licenses
2. **Smart Matching**: Uses fuzzy matching to find licenses even with typos
3. **Template Processing**: Automatically replaces placeholders like `<year>`, `<copyright holders>` with your configuration data
4. **Interactive Prompts**: Asks for any missing information needed to complete the license

### Configuration Management

- **Project Validation**: Ensures configuration matches the current project directory
- **Persistent Storage**: Saves project metadata for consistent license generation
- **Git Integration**: Optionally manages `.gitignore` entries for configuration files

## 🏗️ Architecture

Curator is built with a modular architecture:

- **`main.rs`**: CLI interface using `clap` for argument parsing
- **`config.rs`**: Configuration management and project initialization
- **`license.rs`**: License fetching, processing, and management
- **`tools.rs`**: Utility functions including fuzzy search
- **`lib.rs`**: Library interface for external use

### Dependencies

- **`clap`**: Command-line argument parsing
- **`reqwest`**: HTTP client for fetching licenses
- **`serde_json`**: JSON serialization for configuration
- **`dialoguer`**: Interactive prompts
- **`fuzzy-matcher`**: Fuzzy string matching for license suggestions
- **`crossterm`**: Cross-platform terminal styling
- **`chrono`**: Date handling for copyright years

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major
changes, please open an issue first to discuss what you would like to change.
For more information, please check the [CONTRIBUTING.md](CONTRIBUTING.md) file.

### Development Setup

1. Clone the repository
2. Install Rust (1.70+ recommended)
3. Run tests: `cargo test`
4. Build: `cargo build`
5. Run: `cargo run -- --help`

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [SPDX License List](https://spdx.org/licenses/) for providing the comprehensive license database
- The Rust community for excellent crates and documentation
