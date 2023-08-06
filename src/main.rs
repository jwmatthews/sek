mod cli;
mod commands;
mod config;
use colored::*;

fn main() {
    let matches = cli::cli().get_matches();
    let config_file_name = matches.get_one::<String>("config").expect("required");
    let cfg = config::init(config_file_name).expect("Error parsing configuration file");

    match matches.subcommand() {
        Some(("refresh", _sub_matches)) => {
            println!("{}", "Refresh invoked!".green());
            commands::refresh(cfg);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
