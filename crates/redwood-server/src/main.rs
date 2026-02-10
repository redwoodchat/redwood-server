use clap::{arg, command, value_parser};
use std::path::PathBuf;

fn main() {
    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "The config file to read"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    if let Some(config) = matches.get_one::<PathBuf>("config") {
        println!("config: {}", config.display());
    }

    println!("Ready to run!");
}
