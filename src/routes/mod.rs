use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use askama::{DynTemplate, Template};

use crate::config::Settings;
use crate::error::SsgError;

pub mod about;
pub mod home;

pub struct Routes {
    pub output: PathBuf,
    pub routes: HashMap<String, Box<dyn DynTemplate>>,
}

impl Routes {
    pub fn new(config: Settings) -> Self {
        let output_path = Path::new(&config.output);

        if output_path.exists() {
            fs::remove_dir_all(output_path).expect("Failed to remove existing output directory");
        }
        fs::create_dir_all(output_path).expect("Failed to create output directory");

        Self {
            output: PathBuf::from(&config.output),
            routes: HashMap::new(),
        }
    }

    pub fn build(&self) -> Result<(), SsgError> {
        for (path, template) in &self.routes {
            let mut output_path = self.output.join(path);
            output_path.set_extension("html");
            println!("Output path: {}", self.output.display());
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create parent directories");
            }
            fs::write(&output_path, template.dyn_render()?).expect("Failed to create html file");
            println!("Wrote {} to {}", &path, output_path.display());
        }
        Ok(())
    }

    pub fn route(mut self, path: &str, template: impl Template + 'static) -> Self {
        self.routes.insert(path.to_string(), Box::new(template));
        self
    }
}
