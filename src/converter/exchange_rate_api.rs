use crate::converter::error::converter_request_error::ConverterRequestError;
use crate::converter::Converter;
use json;
use reqwest;

pub struct ExchangeRateApiConverter;

impl Converter for ExchangeRateApiConverter {
  fn convert(&self, from: &str, to: &str) -> () {
    match self.request(from, to) {
      Ok(v) => println!("1 {} = {} {}", from, v, to),
      Err(e) => println!("Failed to get exchange rate: {}", e),
    }
  }
}

impl ExchangeRateApiConverter {
  fn request(&self, from: &str, to: &str) -> Result<f64, ConverterRequestError> {
    return reqwest::blocking::get(&format!(
      "https://api.exchangeratesapi.io/latest?base={}&symbols={}",
      from, to
    ))
    .map_err(|_| ConverterRequestError::FailedToSendRequest)
    .and_then(|r: reqwest::blocking::Response| {
      r.text().map_err(|_| ConverterRequestError::NoResponseBody)
    })
    .and_then(|t: String| json::parse(&t).map_err(|_| ConverterRequestError::FailedToParseResponse))
    .and_then(|j: json::JsonValue| {
      j["rates"][to]
        .as_f64()
        .ok_or(ConverterRequestError::InvalidResponse)
    });
  }
}
