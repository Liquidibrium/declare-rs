use std::fmt;
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