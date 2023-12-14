use serde::{Deserialize, Serialize};
use crate::common::currency::Currency;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclarationEntity {
    pub date: String,
    pub amount: f64,
    pub from: Currency,
    pub to: Currency,
    pub converted_amount: f64,
    pub amount_after_tax: f64,
    pub tax: f64,
    pub tax_amount: f64,
    pub rate: f64,
    pub total: f64,
}