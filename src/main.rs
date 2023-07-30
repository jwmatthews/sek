use clap::Parser;

mod cli;
mod commands;
mod config;

fn main() {
    let cli = cli::Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(cli::Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(cli::Commands::Refresh { list }) => {
            if *list {
                println!("Refresh with list...");
            } else {
                println!("Refresh without list...");
            }
        }
        None => {}
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
        match config::init(config_path) {
            Ok(_cfg) => {
                println!("Parsed cfg")
            }
            Err(e) => {
                println!("Caught error: {}", e.to_string());
            }
        }
    }

    // Continued program logic goes here...
}
