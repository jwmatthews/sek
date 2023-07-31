use clap::{Arg, Command};
use dirs;
use once_cell::sync::OnceCell;
use std::ffi::OsString;

static DEFAULT_CONFIG_FILE: OnceCell<OsString> = OnceCell::new();

pub fn cli() -> Command {
    let home_dir = dirs::home_dir().unwrap();
    DEFAULT_CONFIG_FILE
        .set(OsString::from(format!(
            "{}/.sek/config.yaml",
            home_dir.display()
        )))
        .unwrap();

    Command::new("sek")
        .about("Helper to organize working with multiple Kubernetes environments")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("config")
                .value_name("CONFIG")
                .help("Configuration file path")
                .default_value(DEFAULT_CONFIG_FILE.get().unwrap().as_os_str()),
        )
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("refresh")
                .about("Refresh information about existing Kubernetes environments"),
        )
}
