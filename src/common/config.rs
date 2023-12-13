use std::path::PathBuf;
use clap_serde_derive::ClapSerde;
use serde::{Deserialize, Serialize};
use crate::common::currency::Currency;

#[derive(ClapSerde, Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Currency from
    #[arg(value_enum, long("from"))]
    pub currency_from: Option<Currency>,

    /// Currency to
    #[arg(value_enum, long("to"))]
    pub currency_to: Option<Currency>,

    /// CSV file
    #[arg(value_enum, short('f'), long("file"))]
    pub csv_file: Option<PathBuf>,
    // /// config file
    // #[arg(value_enum, short('c'), long("config"))]
    // pub config_file: Option<PathBuf>,
    /// Tax
    #[arg(short, long, default_value = "0.0")]
    pub tax: f64,
}

