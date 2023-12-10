pub mod config;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use clap_serde_derive::ClapSerde;
use crate::commands::config::Config;
use crate::common::currency::Currency;
use crate::common::output_format::OutputFormat;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    pub name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "config", default_value = "config.yaml")]
    pub config_path: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    /// Rest of arguments
    #[clap(flatten)]
    pub config: <Config as ClapSerde>::Opt,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// show all transactions
    Show {
        /// Output format
        #[arg(short, long, default_value = OutputFormat::CSV.to_string())]
        format: OutputFormat,
    },

    /// initialize the project
    Init {
        /// Currency from
        #[arg(long("from"), default_value = Currency::USD.to_string())]
        currency_from: Currency,

        /// Currency to
        #[arg(long("to"), default_value = Currency::GEL.to_string())]
        currency_to: Currency,

        /// CSV file
        #[arg(short('f'), long, default_value = "transactions.csv")]
        csv_file: PathBuf,

        /// Tax
        #[arg(short, long, default_value = "0.0")]
        tax: f64,
    },

    /// add a new transaction
    Add {
        /// Date
        #[arg(short, long, required = true)]
        date: String,

        /// Amount
        #[arg(short, long, required = true)]
        amount: f64,

        /// Currency from
        #[arg(long("from"))]
        currency_from: Option<Currency>,

        /// Currency to
        #[arg(long("to"))]
        currency_to: Option<Currency>,

        /// Exchange rate
        #[arg(long("rate"))]
        exchange_rate: Option<f64>,
    },

    /// show currency exchange rate
    Exchange {
        /// Currency from
        #[arg(long("from"), default_value = Currency::USD.to_string())]
        currency_from: Currency,

        /// Currency to
        #[arg(long("to"), default_value = Currency::GEL.to_string())]
        currency_to: Currency,

        /// Amount
        #[arg(short, long, default_value = "1.0")]
        amount: f64,

        /// Date
        #[arg(short, long, required = true)]
        date: String,
    },

}