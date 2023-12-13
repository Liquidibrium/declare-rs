use std::path::PathBuf;
use crate::common::consts::csv_path;
use crate::common::currency::Currency;
use crate::declaration::declaration_entity::DeclarationEntity;


const CSV_HEADERS: &str = "date,amount,from,to,rate,tax,amount_after_tax,total";

enum FileCreationStatus {
    Created,
    AlreadyExists,
}

pub struct DeclarationManager {
    csv_file: PathBuf,
    csv_headers: Vec<String>,
}


impl DeclarationManager {
    pub fn create(csv_file: Option<PathBuf>) -> anyhow::Result<Self> {
        let csv_file = csv_file.unwrap_or_else(|| csv_path().unwrap());
        Ok(Self {
            csv_file,
            csv_headers: CSV_HEADERS.split(",").map(|s| s.to_string()).collect(),
        })
    }

    /// create new csv file if it does not exists and add header
    fn init_declaration(&self, overwrite: bool) -> anyhow::Result<FileCreationStatus> {
        if self.csv_file.exists() && !overwrite {
            return Ok(FileCreationStatus::AlreadyExists);
        }

        let mut writer = csv::Writer::from_path(&self.csv_file)?;

        writer.write_record(&self.csv_headers)?;
        Ok(FileCreationStatus::Created)
    }

    pub fn add_new_transaction(&self,
                               data: &str,
                               amount: f64,
                               from: Currency,
                               to: Currency,
                               rate: f64,
                               tax: f64) -> anyhow::Result<f64> {
        self.init_declaration(false)?;

        // read old data, calculate new total, add new row and write to file

        let mut reader = csv::Reader::from_path(&self.csv_file)?;
        let mut writer = csv::Writer::from_path(&self.csv_file)?;

        let records = reader.records().collect::<Result<Vec<_>, _>>()?;
        let mut total = 0.0;
        for record in records.iter() {
            let amount_after_tax = record[6].parse::<f64>()?;
            total += amount_after_tax;
        }

        let amount_after_tax = amount - tax;
        total += amount_after_tax;
        let amount = amount.to_string();
        let from = from.to_string();
        let to = to.to_string();
        let rate = rate.to_string();
        let tax = tax.to_string();
        let amount_after = amount_after_tax.to_string();
        let total_amount = total.to_string();
        let new_record = vec![data,
                              &amount,
                              &from,
                              &to,
                              &rate,
                              &tax,
                              &amount_after,
                              &total_amount];
        writer.write_record(new_record)?;

        Ok(total)
    }

    pub fn show_declaration(&self) -> anyhow::Result<Vec<DeclarationEntity>> {
        let mut reader = csv::Reader::from_path(&self.csv_file)?;
        let records = reader.records().collect::<Result<Vec<_>, _>>()?;

        return records.iter().map(|record| {
            let date = record[0].to_string();
            let amount = record[1].parse::<f64>()?;
            let from = record[2].parse::<Currency>()?;
            let to = record[3].parse::<Currency>()?;
            let rate = record[4].parse::<f64>()?;
            let tax = record[5].parse::<f64>()?;
            let amount_after_tax = record[6].parse::<f64>()?;
            let total = record[7].parse::<f64>()?;

            Ok(DeclarationEntity {
                date,
                amount,
                from,
                to,
                rate,
                tax,
                amount_after_tax,
                total,
            })
        }).collect::<Result<Vec<_>, _>>();
    }
}