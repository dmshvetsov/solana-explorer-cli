use std::{error::Error, fmt::Debug};

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
}
