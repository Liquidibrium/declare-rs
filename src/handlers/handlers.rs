

use std::path::PathBuf;
use chrono::NaiveDate;
use crate::commands::config::Config;
use crate::common::currency::Currency;
use crate::common::output_format::OutputFormat;
use crate::exchange::exchanger::{Exchanger, NbgExchanger};


pub fn save_config(config: &Config, config_path: &PathBuf) -> anyhow::Result<()> {
    let config_file = std::fs::File::create(config_path)?;
    serde_yaml::to_writer(config_file, config)?;
    Ok(())
}

pub fn show_transactions(config: &Config, format: &OutputFormat) -> anyhow::Result<()> {
    // read the data

    // print the data

    Ok(())
}


pub fn add_new_transaction(config: &Config,
                           date: &String,
                           amount: &f64,
                           from: &Option<Currency>,
                           to: &Option<Currency>,
                           exchange_rate: &Option<f64>) -> anyhow::Result<()> {
    Ok(())
}

pub fn print_exchange_rate(config: &Config,
                           from: &Currency,
                           to: &Currency,
                           date: &String,
                           amount: &f64,
) -> anyhow::Result<()> {
    println!("{} {} = ? {}", amount, from, to);
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();

    let exchange_rate_amount = NbgExchanger::new().exchange_rate(from, to, date, *amount)?;
    println!("{} {} = {} {}", amount, from, exchange_rate_amount, to);

    Ok(())
}

