use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NbgExchangeResponse {
    pub date: String,
    pub currencies: Vec<Currency>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    #[serde(rename = "rateFormated")]
    pub rate_formatted: String,

    pub date: String,
    pub code: String,
    pub quantity: i64,
    pub rate: f64,
    pub name: String,

    #[serde(rename = "validFromDate")]
    pub valid_from_date: String,
    pub diff: f64,

    #[serde(rename = "diffFormated")]
    pub diff_formatted: String,
}
