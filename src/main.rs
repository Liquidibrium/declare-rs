pub mod commands;
pub mod handlers;
pub mod exchange;
pub mod common;
mod declaration;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use clap::{Parser};
use clap_serde_derive::ClapSerde;
use crate::commands::{Cli, Commands};
use crate::common::config::{ClapSerdeOptionalConfig, Config};
use crate::common::consts::config_path;
use crate::handlers::handlers::{add_new_transaction, open_cvs_file, print_exchange_rate, save_config, show_transactions};


fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Show { format }) => {
            let config_path = config_path().expect("Could not get config path");
            let config = get_config(config_path).expect("Could not find config file");
            show_transactions(&config, format).unwrap_or_else(|err| {
                eprintln!("Error showing transactions: {}", err);
            });
        }
        Some(Commands::Open {}) => {
            let config_path = config_path().expect("Could not get config path");
            let config = get_config(config_path).expect("Could not find config file");
            open_cvs_file(&config).unwrap_or_else(|err| {
                eprintln!("error opening file: {}", err);
            });
        }
        Some(Commands::Init { config }) => {
            let config_path = config_path().expect("Could not get config path");
            let mut x = config.clone();
            let config = merge_config(config_path.clone(), &mut x)
                .expect("Could not find config file");
            save_config(&config, &config_path).unwrap_or_else(|err| {
                eprintln!("Error saving config: {}", err);
            });
        }
        Some(Commands::Add { date, amount, exchange_rate, config }) => {
            let config_path = config_path().expect("Could not get config path");
            let config = merge_config(config_path, &mut config.clone()).expect("Could not find config file");
            add_new_transaction(&config, date, amount, exchange_rate).unwrap_or_else(|err| {
                eprintln!("Error adding new transaction: {}", err);
            });
        }
        Some(Commands::Exchange { currency_from, currency_to, amount, date }) => {
            print_exchange_rate(currency_from, currency_to, date, amount).unwrap_or_else(|err| {
                eprintln!("Error printing exchange rate: {}", err);
            });
        }
        None => {}
    }
}


fn get_config(config_path: PathBuf) -> anyhow::Result<Config> {
    let config_file = File::open(config_path)?;
    let config = serde_yaml::from_reader::<_, <Config as ClapSerde>::Opt>(BufReader::new(config_file))?;
    Ok(Config::from(config))
}

fn merge_config(config_path: PathBuf, cli_config: &mut ClapSerdeOptionalConfig) -> anyhow::Result<Config> {
    let config_result = get_config(config_path);
    let config = if let Ok(config) = config_result {
        // merge config already parsed from clap
        Config::from(config).merge(cli_config)
    } else {
        // If there is not config file return only config parsed from clap
        Config::from(cli_config)
    };
    Ok(config)
}



