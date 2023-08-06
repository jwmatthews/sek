use colored::*;
use dirs;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub sek_home: String,
    pub agnosticd_resource_dirs: Vec<String>,
}

pub fn get_default_config() -> Config {
    let home_dir = dirs::home_dir().unwrap();
    let cfg = Config {
        sek_home: format!("{}/.sek", &home_dir.display()),
        agnosticd_resource_dirs: vec![
            format!("{}/.agnosticd", &home_dir.display()),
            format!("{}/Desktop/share/agnosticd", &home_dir.display()),
        ],
    };
    return cfg;
}

pub fn init(config_file_name: &str) -> Result<Config, io::Error> {
    let config_file_path = Path::new(config_file_name);
    let mut cfg: Config = get_default_config();
    if config_file_path.is_file() {
        println!(
            "Attempting to open '{}' to parse for configuration.",
            config_file_path.display().to_string().cyan()
        );
        cfg = parse(config_file_path).expect(&format!(
            "Failed to parse: {}",
            &config_file_path.display().to_string().red()
        ))
    }
    ensure_or_create_dir(&cfg.sek_home);

    if !config_file_path.is_file() {
        // Our config file was empty, so let's create a default config and write it to disk
        // so easier to modify on future runs
        let yaml = serde_yaml::to_string(&cfg).expect("Failed to deserialize");
        println!(
            "Attempting to write config data \n'{}'\n to\n'{}'",
            &yaml.to_string().yellow(),
            config_file_path.display().to_string().cyan()
        );
        let mut output = File::create(config_file_path)?;
        write!(output, "{}", yaml)?;
    }
    return Ok(cfg);
}

pub fn parse(config_file_path: &Path) -> Result<Config, serde_yaml::Error> {
    let yaml = fs::read_to_string(&config_file_path)
        .expect(&format!("Unable to open: {}", &config_file_path.display()));
    let cfg: Config = serde_yaml::from_str(&yaml)?;
    return Ok(cfg);
}

pub fn ensure_or_create_dir(dir_name: &str) {
    let p = Path::new(dir_name);
    if !p.is_dir() {
        println!("Creating directory: {}", dir_name.cyan());
        fs::create_dir_all(p).expect("Directory create failed");
    }
}
