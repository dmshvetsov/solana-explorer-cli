use crate::output::{output_json, output_raw_struct, Output, OutputFormat};

pub struct Page {
    content: Vec<Box<dyn Output>>,
    format: OutputFormat,
}

impl Page {
    pub fn new(format: OutputFormat) -> Self {
        Page {
            content: Vec::new(),
            format,
        }
    }

    pub fn add(self: &mut Self, item: impl Output + 'static) {
        self.content.push(Box::new(item));
    }

    pub fn display(self: &Self) {
        match self.format {
            OutputFormat::AsStruct => {
                for box_s in self.content {
                    println!(
                        "se::{}::{:#?}",
                        box_s.struct_name(),
                        box_s.downcast::<Output>()
                    );
                }
            }
            OutputFormat::AsJson => {
                for box_s in self.content {
                    println!("se::{} {}", box_s.struct_name(), box_s.to_json());
                }
            }
        }
    }
}
