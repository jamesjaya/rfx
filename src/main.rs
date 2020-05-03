extern crate clap;
extern crate reqwest;

mod converter;
mod converter_registry;
use clap::{load_yaml, App};

use converter::Converter;

fn main() {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from(yaml).get_matches();

  let from = matches.value_of("from").unwrap();
  let to = matches.value_of("to").unwrap();

  let registry = converter_registry::ConverterRegistry {};
  registry.get_converter(None).convert(from, to);
}
