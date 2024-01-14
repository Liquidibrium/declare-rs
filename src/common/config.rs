use std::fmt::{Debug, Formatter};
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
    #[arg(short, long)]
    pub tax: Option<f64>,
}


impl Clone for ClapSerdeOptionalConfig {
    fn clone(&self) -> Self {
        Self {
            currency_from: self.currency_from.clone(),
            currency_to: self.currency_to.clone(),
            csv_file: self.csv_file.clone(),
            tax: self.tax.clone(),
        }
    }
}


impl Debug for ClapSerdeOptionalConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClapSerdeOptionalConfig")
            .field("currency_from", &self.currency_from)
            .field("currency_to", &self.currency_to)
            .field("csv_file", &self.csv_file)
            .field("tax", &self.tax)
            .finish()
    }
}