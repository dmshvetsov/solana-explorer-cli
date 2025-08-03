use crate::output::{Output, OutputFormat};

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

    pub fn add(&mut self, item: impl Output + 'static) {
        self.content.push(Box::new(item));
    }

    pub fn display(&self) {
        match self.format {
            OutputFormat::AsStruct => {
                for box_s in &self.content {
                    let output = box_s.to_raw_struct();
                    println!("se::{output}");
                }
            }
            OutputFormat::AsJson => {
                for box_s in &self.content {
                    // TODO: maybe page will be top level JSON "object" {}
                    // and all elements will be "<name call value>": <to_json call value>
                    // it must produce valid JSON so it can be copied or piped to other utils that works with JSON like `jq`
                    println!("{}", box_s.to_json());
                }
            }
        }
    }
}
