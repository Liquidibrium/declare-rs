use chrono::NaiveDate;
use crate::common::currency::Currency;
use crate::exchange::nbg_exchange_response::NbgExchangeResponse;

pub trait Exchanger {
    fn exchange_rate(&self, from: &Currency, to: &Currency, date: NaiveDate, amount: f64) -> Result<f64, anyhow::Error>;
}


pub struct NbgExchanger {
    client: reqwest::blocking::Client,
}

impl NbgExchanger {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new()
        }
    }
}


impl Exchanger for NbgExchanger {
    fn exchange_rate(&self, from: &Currency, to: &Currency, date: NaiveDate, amount: f64) -> Result<f64, anyhow::Error> {
        if to != &Currency::GEL {
            return Err(anyhow::anyhow!("NbgExchanger | nbg.gov.ge does not support exchange to {}", to));
        }
        if from == to {
            return Ok(amount);
        }

        let date = date.format("[year]-[month]-[day]").to_string();
        let url = format!("https://nbg.gov.ge/gw/api/ct/monetarypolicy/currencies/ka/json/?currencies={}&date={}", from, date);
        let response = self.client.get(url).send()?;
        let response = response.json::<Vec<NbgExchangeResponse>>()?;


        if let Some(response) = response.get(0) {
            let currency = response.currencies.iter().find(|c| c.code == from.to_string())
                .ok_or(anyhow::anyhow!("currency not found"))?;
            let rate = currency.rate;
            return Ok(rate * amount);
        }

        Err(anyhow::anyhow!("currency not found"))
    }
}