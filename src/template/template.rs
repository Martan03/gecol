use std::path::Path;

use minijinja::{Environment, value::ViaDeserialize};
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    theme::{Color, Theme},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template<P1, P2>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    pub template: P1,
    pub target: P2,
}

impl<P1, P2> Template<P1, P2>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    /// Creates new template with `template` as source and `target` as the
    /// build destination.
    pub fn new(template: P1, target: P2) -> Self {
        Self { template, target }
    }

    /// Builds the template in the `template` dir and saves it to `target`.
    pub fn build(&self, theme: &Theme) -> Result<(), Error> {
        let content = std::fs::read_to_string(&self.template)?;

        let mut env = Environment::new();

        fn hex(color: ViaDeserialize<Color>) -> String {
            color.0.hex()
        }
        env.add_filter("hex", hex);

        env.add_template("template", &content)?;

        let template = env.get_template("template")?;
        let built = template.render(&theme)?;

        std::fs::write(&self.target, built)?;
        Ok(())
    }
}
