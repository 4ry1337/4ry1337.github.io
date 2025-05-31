use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use askama::{DynTemplate, Template};
use tracing::info;

use crate::config::Settings;
use crate::error::Error;

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

    pub fn build(&self) -> Result<(), Error> {
        if !Path::new(&self.output).exists() {
            fs::create_dir(&self.output)?;
        }
        for (path, template) in &self.routes {
            let mut output_path = PathBuf::from(&self.output).join(path);
            output_path.set_extension("html");
            fs::write(&output_path, template.dyn_render()?)?;
            info!("Wrote {} to {}", &path, output_path.display());
        }
        Ok(())
    }

    pub fn route(self, path: &str, template: impl Template + 'static) -> Self {
        let mut routes = HashMap::from(self.routes);
        routes.insert(path.to_string(), Box::new(template));
        Self {
            output: self.output,
            routes,
        }
    }
}
