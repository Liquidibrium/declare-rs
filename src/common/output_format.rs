use std::fmt;
use std::str::FromStr;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    CSV,
    JSON,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OutputFormat::CSV => write!(f, "CSV"),
            OutputFormat::JSON => write!(f, "JSON"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CSV" => Ok(OutputFormat::CSV),
            "JSON" => Ok(OutputFormat::JSON),
            _ => Err(format!("unknown output format: {}", s)),
        }
    }
}

