use std::fs::OpenOptions;
use std::path::PathBuf;
use crate::common::consts::csv_path;
use crate::common::currency::Currency;
use crate::declaration::declaration_entity::DeclarationEntity;


const CSV_HEADERS: &str = "date,amount,from,to,converted_amount,amount_after_tax,tax,tax_amount,total";

// pub enum FileCreationStatus {
//     Created,
//     AlreadyExists,
// }

pub struct DeclarationManager {
    csv_file: PathBuf,
    csv_headers: Vec<&'static str>,
}


impl DeclarationManager {
    pub fn create(csv_file: Option<PathBuf>) -> anyhow::Result<Self> {
        let csv_file = csv_file.unwrap_or_else(|| csv_path().unwrap());
        Ok(Self {
            csv_file,
            csv_headers: CSV_HEADERS.split(",").collect(),
        })
    }

    // /// create new csv file if it does not exists and add header
    // fn init_declaration(&self, overwrite: bool) -> anyhow::Result<FileCreationStatus> {
    //     if self.csv_file.exists() && !overwrite {
    //         println!("File already exists: {:?}", self.csv_file);
    //         return Ok(FileCreationStatus::AlreadyExists);
    //     }
    //     println!("Creating file: {:?}, with headers {:?}", self.csv_file, self.csv_headers);
    //
    //     let mut writer = csv::Writer::from_path(&self.csv_file)?;
    //
    //     writer.write_record(self.csv_headers.clone())?;
    //     writer.flush()?;
    //     Ok(FileCreationStatus::Created)
    // }

    pub fn add_new_transaction(&self,
                               date: &str,
                               amount: f64,
                               from: Currency,
                               to: Currency,
                               converted_amount: f64,
                               amount_after_tax: f64,
                               tax: f64,
                               tax_amount: f64) -> anyhow::Result<f64> {
        let records = self.get_existing_declarations()?;
        println!("records size: {:?}", records.len());

        // read old data, calculate new total, add new row and write to file
        let mut writer = if records.is_empty() {

            // If the file is empty, create a new file with a header
            csv::WriterBuilder::new()
                .has_headers(true)
                .from_path(&self.csv_file)?
        } else {
            // If the file already has data, open it in append mode
            let file = OpenOptions::new().append(true).open(self.csv_file.clone())?;
            csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(file)
        };

        let mut total = 0.0;
        for record in records.iter() {
            total += record.amount_after_tax;
        }

        total += amount_after_tax;

        writer.serialize(DeclarationEntity {
            date: date.to_string(),
            amount,
            from,
            to,
            converted_amount,
            tax,
            tax_amount,
            amount_after_tax,
            total,
        })?;

        Ok(total)
    }

    pub fn get_existing_declarations(&self) -> anyhow::Result<Vec<DeclarationEntity>> {
        if !self.csv_file.exists() {
            println!("File does not exists: {:?}", self.csv_file);
            return Ok(vec![]);
        }
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(&self.csv_file)?;

        return Ok(reader.deserialize().collect::<Result<Vec<DeclarationEntity>, _>>()?);
    }
}