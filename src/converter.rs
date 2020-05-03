pub mod error;
pub mod exchange_rate_api;

pub trait Converter {
  fn convert(&self, from: &str, to: &str) -> ();
}
