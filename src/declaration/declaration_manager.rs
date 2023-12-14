use std::fs::OpenOptions;
use std::path::PathBuf;
use crate::common::consts::csv_path;
use crate::declaration::declaration_entity::DeclarationEntity;


pub struct DeclarationManager {
    csv_file: PathBuf,
}


impl DeclarationManager {
    pub fn create(csv_file: Option<PathBuf>) -> anyhow::Result<Self> {
        let csv_file = csv_file.unwrap_or_else(|| csv_path().unwrap());
        println!("CSV file: {:?}", csv_file);
        Ok(Self {
            csv_file,
        })
    }


    pub fn add_new_transaction(&self,
                               should_initialize: bool,
                               declaration_entity: DeclarationEntity) -> anyhow::Result<()> {
        // read old data, calculate new total, add new row and write to file
        let mut writer = if should_initialize {

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

        writer.serialize(declaration_entity)?;

        Ok(())
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