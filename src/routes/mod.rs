use std::collections::HashMap;
use std::fs;
use std::path::Path;

use askama::{DynTemplate, Template};

use crate::config::Settings;
use crate::error::SsgError;

pub mod about;
pub mod home;

pub struct Routes {
    pub output: String,
    pub routes: HashMap<String, Box<dyn DynTemplate>>,
}

impl Routes {
    pub fn new(config: Settings) -> Self {
        Self {
            output: config.output,
            routes: HashMap::new(),
        }
    }

    pub fn build(&self) -> Result<(), SsgError> {
        for (path, template) in &self.routes {
            let output_path = Path::new(&format!("{}{}", self.output, path)).join("index.html");
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&output_path, template.dyn_render()?)?;
            println!("Wrote {} to {}", &path, output_path.display());
        }
        Ok(())
    }

    pub fn route(mut self, path: &str, template: impl Template + 'static) -> Self {
        self.routes.insert(path.to_string(), Box::new(template));
        self
    }
}
