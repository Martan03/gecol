use std::path::Path;

use minijinja::{Environment, Value, context};
use serde::{Deserialize, Serialize};

use crate::{error::Error, theme::Theme};

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

        env.add_template("template", &content)?;

        let template = env.get_template("template")?;
        let ctx = context! {
            primary => Value::from_object(theme.primary),
            secondary => Value::from_object(theme.secondary),
            background => Value::from_object(theme.background),
            surface => Value::from_object(theme.surface),
            border => Value::from_object(theme.border),
            foreground => Value::from_object(theme.foreground),
            muted => Value::from_object(theme.muted),
            success => Value::from_object(theme.success),
            warning => Value::from_object(theme.warning),
            error => Value::from_object(theme.error),
        };
        let built = template.render(ctx)?;

        std::fs::write(&self.target, built)?;
        Ok(())
    }
}
