use std::fs::create_dir_all;
use std::path::PathBuf;

const LOCAL_DIRECTORY: &str = ".config/declare";

const CONFIG_FILE_NAME: &str = "config.yaml";

const CSV_FILE_NAME: &str = "declaration.csv";


pub fn config_path() -> anyhow::Result<PathBuf> {
    // get the home directory
    let home_dir = dirs::home_dir().ok_or(anyhow::anyhow!("Could not find home directory"))?;

    // create the local directory
    let local_dir = home_dir.join(LOCAL_DIRECTORY);
    create_dir_all(local_dir.clone())?;

    // create the config file
    let config_file = local_dir.join(CONFIG_FILE_NAME);
    Ok(config_file)
}


pub fn csv_path() -> anyhow::Result<PathBuf> {
    // get the home directory
    let home_dir = dirs::home_dir().ok_or(anyhow::anyhow!("Could not find home directory"))?;

    // create the local directory
    let local_dir = home_dir.join(LOCAL_DIRECTORY);

    // create the csv file
    let csv_file = local_dir.join(CSV_FILE_NAME);

    Ok(csv_file)
}