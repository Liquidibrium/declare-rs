pub mod commands;
pub mod handlers;
pub mod exchange;
pub mod common;
mod declaration;

use std::fs::File;
use std::io::BufReader;
use clap::{Parser};
use clap_serde_derive::ClapSerde;
use crate::commands::{Cli, Commands};
use crate::common::config::Config;
use crate::common::consts::config_path;
use crate::handlers::handlers::{add_new_transaction, open_cvs_file, print_exchange_rate, save_config, show_transactions};


fn main() {
    let mut cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    // Get config file
    let config_path = config_path().unwrap();
    println!("Config path: {:?}", config_path);
    let config = if let Ok(f) = File::open(config_path.clone()) {
        // Parse config with serde
        match serde_yaml::from_reader::<_, <Config as ClapSerde>::Opt>(BufReader::new(f)) {
            // merge config already parsed from clap
            Ok(config) => Config::from(config).merge(&mut cli.config),
            Err(err) => panic!("Error in configuration file:\n{}", err),
        }
    } else {
        // If there is not config file return only config parsed from clap
        Config::from(&mut cli.config)
    };

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Show { format }) => {
            show_transactions(&config, format).unwrap_or_else(|err| {
                eprintln!("Error showing transactions: {}", err);
            });
        }
        Some(Commands::Open {}) => {
            open_cvs_file(&config).unwrap_or_else(|err| {
                eprintln!("error opening file: {}", err);
            });
        }
        Some(Commands::Init {}) => {
            save_config(&config, &config_path).unwrap_or_else(|err| {
                eprintln!("Error saving config: {}", err);
            });
        }
        Some(Commands::Add { date, amount, currency_from, currency_to, exchange_rate }) => {
            add_new_transaction(&config, date, amount, currency_from, currency_to, exchange_rate).unwrap_or_else(|err| {
                eprintln!("Error adding new transaction: {}", err);
            });
        }
        Some(Commands::Exchange { currency_from, currency_to, amount, date }) => {
            print_exchange_rate(&config, currency_from, currency_to, date, amount).unwrap_or_else(|err| {
                eprintln!("Error printing exchange rate: {}", err);
            });
        }
        None => {}
    }

    // Continued program logic goes here...
}



