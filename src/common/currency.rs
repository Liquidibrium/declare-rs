use std::fmt;
use std::str::FromStr;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Deserialize, Serialize)]
pub enum Currency {
    USD,
    GEL,
    EUR,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::GEL => write!(f, "GEL"),
        }
    }
}

impl FromStr for Currency {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "usd" => Ok(Currency::USD),
            "eur" => Ok(Currency::EUR),
            "gel" => Ok(Currency::GEL),
            _ => Err(anyhow::anyhow!("Invalid currency")),
        }
    }
}