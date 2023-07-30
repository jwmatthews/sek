use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "sek")]
#[command(author = "John W. Matthews <jwmatthews@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Helps manage Shell Environments for Kubernetes clusters", long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    pub name: Option<String>,

    // QUESTION:  How can I set a default value
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", default_value = "~/.sek/config")]
    // Need to set the default value for config with let home_dir = dirs::home_dir().unwrap()
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Refresh {
        #[arg(short, long)]
        list: bool,
    },
}
