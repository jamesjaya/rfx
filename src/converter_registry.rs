use crate::converter::exchange_rate_api::ExchangeRateApiConverter;
use crate::converter::Converter;

pub struct ConverterRegistry;

impl ConverterRegistry {
  pub fn get_converter(&self, _id: Option<&str>) -> impl Converter {
    return ExchangeRateApiConverter {};
  }
}
