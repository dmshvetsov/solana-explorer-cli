use serde::Serialize;
use std::fmt::Debug;
use std::error::Error;

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

pub fn output_raw_struct<T: Debug + Output>(instance: T) {
    println!("{}::{:#?}", instance.struct_name::<T>(), instance);
}

pub fn output_json<T: Output + Serialize>(instance: T) {
    println!("{} {}", instance.struct_name::<T>(), instance.to_json(),);
}

/// methods required to output a struct from the CLI
pub trait Output {
    fn struct_name<T>(&self) -> String {
        let type_name = std::any::type_name::<T>().split("::");
        let size = type_name.clone().count();
        let mut type_prefix = type_name.take(size - 1).collect::<Vec<&str>>().join("::");
        if type_prefix.starts_with('&') {
            type_prefix = type_prefix.strip_prefix('&').unwrap().to_string();
        }
        type_prefix
    }

    fn to_json(&self) -> String
    where
        Self: Serialize,
    {
        serde_json::to_string_pretty(self).unwrap()
    }
}
