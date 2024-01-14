use std::path::PathBuf;
use chrono::NaiveDate;
use comfy_table::Table;
use crate::common::config::Config;
use crate::common::consts::csv_path;
use crate::common::currency::Currency;
use crate::common::output_format::OutputFormat;
use crate::declaration::declaration_entity::DeclarationEntity;
use crate::declaration::declaration_manager::DeclarationManager;
use crate::exchange::exchanger::{Exchanger, NbgExchanger};

const DATE_FORMATS: [&str; 6] = [
    "%Y-%m-%d",
    "%Y.%m.%d",
    "%Y/%m/%d",
    "%d-%m-%y",
    "%d.%m.%Y",
    "%d/%m/%y",
];

pub fn save_config(config: &Config, config_path: &PathBuf) -> anyhow::Result<()> {
    let config_file = std::fs::File::create(config_path)?;
    serde_yaml::to_writer(config_file, config)?;
    Ok(())
}

pub fn open_cvs_file(config: &Config) -> anyhow::Result<()> {
    let csv_file = config.csv_file.clone().unwrap_or_else(|| csv_path().unwrap());

    opener::open(csv_file)?;
    Ok(())
}

pub fn show_transactions(config: &Config, format: &OutputFormat) -> anyhow::Result<()> {
    println!("Show format: {:?} -> {}, {:?}", format, OutputFormat::CSV.to_string(), config);
    let declaration_manager = DeclarationManager::create(config.csv_file.clone())?;
    // read the data
    let data = declaration_manager.get_existing_declarations()?;
    // print the data

    match format {
        OutputFormat::CSV => {
            let table = generate_table(&data);
            println!("{table}");
        }
        OutputFormat::JSON => {
            println!("{}", serde_json::to_string(&data)?);
        }
    }
    Ok(())
}

fn generate_table(data: &Vec<DeclarationEntity>) -> String {
    let mut table = Table::new();
    table
        .set_header(vec!["#", "time", "amount from", "converted to", "tax", "tax amount", "amount after tax", "total"]);

    for (i, record) in data.iter().enumerate() {
        table.add_row(vec![
            format!("{}", i + 1),
            record.date.to_string(),
            format!("{} {}", record.amount, record.from),
            format!("{} {}", record.converted_amount, record.to),
            format!("{} %", record.tax),
            format!("{} {}", record.tax_amount, record.to),
            format!("{} {}", record.amount_after_tax, record.to),
            format!("{} {}", record.total, record.to),
        ]);
    }

    return table.to_string();
}


pub fn add_new_transaction(config: &Config,
                           date: &String,
                           amount: &f64,
                           exchange_rate: &Option<f64>) -> anyhow::Result<()> {
    let declaration_manager = DeclarationManager::create(config.csv_file.clone())?;

    let from = config.currency_from.unwrap_or(Currency::USD);
    let to = config.currency_to.unwrap_or(Currency::GEL);
    let native_date = parse_date(date, DATE_FORMATS.to_vec())?;
    let exchange_rate = exchange_rate.unwrap_or_else(|| {
        NbgExchanger::new().exchange_rate(&from, &to, native_date, 1.0).unwrap()
    });

    println!("Exchange rate {} {} = {} {}", 1.0, from, exchange_rate, to);

    let tax = config.tax.unwrap_or(0.0);
    let converted_amount = exchange_rate * amount;
    let tax_amount = converted_amount * tax / 100.0;
    let amount_after_tax = converted_amount - tax_amount;
    println!("converted {} {} = {} {} - tax: {}% = {} {} - amount after tax: {} {}",
             amount, from, converted_amount, to, tax, tax_amount, to, amount_after_tax, to);
    let records = declaration_manager.get_existing_declarations()?;

    let total = match records.last() {
        Some(last) => last.total,
        None => 0.0
    } + converted_amount;

    declaration_manager.add_new_transaction(records.is_empty(),
                                            DeclarationEntity {
                                                date: date.to_string(),
                                                amount: *amount,
                                                from,
                                                to,
                                                converted_amount,
                                                tax,
                                                tax_amount,
                                                amount_after_tax,
                                                total,
                                                rate: exchange_rate,
                                            })?;
    println!("Total: {} {} ", total, to);
    Ok(())
}

fn parse_date(date: &String, formats: Vec<&str>) -> anyhow::Result<NaiveDate> {
    for format in formats {
        if let Ok(date) = NaiveDate::parse_from_str(date, format) {
            return Ok(date);
        }
    }
    Err(anyhow::anyhow!("Could not parse date"))
}


pub fn print_exchange_rate(
    from: &Currency,
    to: &Currency,
    date: &String,
    amount: &f64,
) -> anyhow::Result<()> {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let exchange_rate_amount = NbgExchanger::new().exchange_rate(from, to, date, *amount)?;

    println!("{} {} = {} {} - Date: {}", amount, from, exchange_rate_amount, to, date);

    Ok(())
}

