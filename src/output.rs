use std::error::Error;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum OutputFormat {
    AsStruct,
    AsJson,
}

impl clap::ValueEnum for OutputFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::AsStruct, Self::AsJson]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::AsStruct => Some(clap::builder::PossibleValue::new("raw")),
            Self::AsJson => Some(clap::builder::PossibleValue::new("json")),
        }
    }
}

pub fn print_struct<T: Debug>(data_struct: T) {
    let type_name = std::any::type_name::<T>().split("::");
    let size = type_name.clone().count();
    let mut type_prefix = type_name.take(size - 1).collect::<Vec<&str>>().join("::");
    if type_prefix.starts_with('&') {
        type_prefix = type_prefix.strip_prefix('&').unwrap().to_string();
    }
    println!("{}::{:#?}", type_prefix, data_struct);
}

pub fn print_warning(msg: &str) {
    println!("{}", msg);
}

pub fn print_error<T: Error>(err: T) {
    println!("{}", err);
}

/// methods required to output a struct from the CLI
pub trait Output {
    fn struct_name(&self) -> String;

    // TODO: rename it to pretty or something like this as i do prettyfication of Strings, Pubkeys
    // keep this as default format
    // add raw output as non defaul
    /// Debug trait must be implemented, it is used to output raw structs
    fn to_raw_struct(&self) -> String;

    /// Serde serialize must be implemented, it is used to serialize structs to JSON
    fn to_json(&self) -> String;
}
