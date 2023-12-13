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
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }
    // Get config file
    let config_path = config_path().unwrap();
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
            println!("Show format: {:?}", format);
            show_transactions(&config, format).unwrap();
        }
        Some(Commands::Open {}) => {
            open_cvs_file(&config).unwrap();
        }
        Some(Commands::Init {}) => {
            save_config(&config, &config_path).unwrap();
        }
        Some(Commands::Add { date, amount, currency_from, currency_to, exchange_rate }) => {
            println!("Add date: {:?}", date);
            println!("Add amount: {:?}", amount);
            println!("Add currency_from: {:?}", currency_from);
            println!("Add currency_to: {:?}", currency_to);
            println!("Add exchange_rate: {:?}", exchange_rate);
            add_new_transaction(&config, date, amount, currency_from, currency_to, exchange_rate).unwrap();
        }
        Some(Commands::Exchange { currency_from, currency_to, amount, date }) => {
            println!("Exchange currency_from: {:?}", currency_from);
            println!("Exchange currency_to: {:?}", currency_to);
            println!("Exchange amount: {:?}", amount);
            println!("Exchange date: {:?}", date);
            print_exchange_rate(&config, currency_from, currency_to, date, amount).unwrap();
        }
        None => {}
    }

    // Continued program logic goes here...
}



