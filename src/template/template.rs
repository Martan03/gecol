use std::path::{Path, PathBuf};

use minijinja::Environment;
use serde::{Deserialize, Serialize};

use crate::{error::Error, template::jinja_context, theme::Theme};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub source: PathBuf,
    pub target: PathBuf,
}

impl Template {
    /// Creates new template with `template` as source and `target` as the
    /// build destination.
    pub fn new<P1, P2>(source: P1, target: P2) -> Self
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        Self {
            source: source.as_ref().to_owned(),
            target: target.as_ref().to_owned(),
        }
    }

    /// Builds the template at `source` and saves it to `target`.
    pub fn build(&self, theme: &Theme) -> Result<(), Error> {
        let content = std::fs::read_to_string(&self.source)?;

        let mut env = Environment::new();
        env.add_template("template", &content)?;

        let source = self.source.to_string_lossy();
        let template = env.get_template(&source)?;
        let ctx = jinja_context(theme.clone());
        let built = template.render(ctx)?;

        if let Some(parent) = self.target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.target, built)?;

        Ok(())
    }
}
