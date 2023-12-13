use std::path::PathBuf;
use chrono::NaiveDate;
use crate::common::config::Config;
use crate::common::currency::Currency;
use crate::common::output_format::OutputFormat;
use crate::declaration::declaration_manager::DeclarationManager;
use crate::exchange::exchanger::{Exchanger, NbgExchanger};


pub fn save_config(config: &Config, config_path: &PathBuf) -> anyhow::Result<()> {
    let config_file;
    if config_path.exists() {
        config_file = std::fs::File::open(config_path)?;
    } else {
        config_file = std::fs::File::create(config_path)?;
    }
    serde_yaml::to_writer(config_file, config)?;
    Ok(())
}

pub fn open_cvs_file(config: &Config) -> anyhow::Result<()> {
    let csv_file = config.csv_file.unwrap_or_else(|| {
        crate::common::consts::csv_path().unwrap()
    });
    opener::open(csv_file)?;
    Ok(())
}

pub fn show_transactions(config: &Config, format: &OutputFormat) -> anyhow::Result<()> {
    println!("Show format: {:?} -> {}, {:?}", format, OutputFormat::CSV.to_string(), config);
    let declaration_manager = DeclarationManager::create(config.clone())?;
    // read the data
    let data = declaration_manager.show_declaration()?;
    // print the data

    match format {
        OutputFormat::CSV => {
            println!("CSV");
            for row in data {
                println!("{:?}", row);
            }
        }
        OutputFormat::JSON => {
            println!("{}", serde_json::to_string(&data)?);
        }
    }
    Ok(())
}


pub fn add_new_transaction(config: &Config,
                           date: &String,
                           amount: &f64,
                           from: &Option<Currency>,
                           to: &Option<Currency>,
                           exchange_rate: &Option<f64>) -> anyhow::Result<()> {
    let declaration_manager = DeclarationManager::create(config.clone())?;

    let from = from.unwrap_or(config.currency_from.unwrap_or(Currency::USD));
    let to = to.unwrap_or(config.currency_to.unwrap_or(Currency::GEL));
    let native_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
    let exchange_rate = exchange_rate.unwrap_or_else(|| {
        NbgExchanger::new().exchange_rate(&from, &to, native_date, *amount).unwrap()
    });
    let tax = config.tax;

    let total = declaration_manager.add_new_transaction(date, *amount, from, to, exchange_rate, tax)?;
    println!("Total: {}", total);
    Ok(())
}

pub fn print_exchange_rate(_config: &Config,
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

