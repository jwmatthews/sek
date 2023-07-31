mod cli;
mod commands;
mod config;

fn main() {
    let matches = cli::cli().get_matches();
    let config_file_name = matches.get_one::<String>("config").expect("required");

    match config::init(config_file_name) {
        Ok(_cfg) => {
            println!("Parsed cfg")
        }
        Err(e) => {
            println!("Caught error: {}", e.to_string());
        }
    }

    match matches.subcommand() {
        Some(("refresh", sub_matches)) => {
            println!("Refresh invoked!");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
