use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use minijinja::{Environment, ErrorKind, Value, context};

use crate::{error::Error, template::template_struct::run_hook, theme::Theme};

pub mod color;
mod template_struct;

pub use template_struct::Template;

/// Builds all the given templates with the given theme.
///
/// This is prefered way of building multiple templates compared to the
/// [`Template::build`](crate::template::Template::build), because it reuses
/// the same building environment.
///
/// It runs all the hooks after copying all the templates. This is usefull
/// if you have multiple files (such as waybar config and color), but you
/// don't want to run the hook twice.
pub fn build_templates(
    templates: &HashMap<String, Template>,
    theme: Theme,
) -> Result<(), Error> {
    let mut env = Environment::new();
    env.set_loader(|name| match std::fs::read_to_string(name) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(minijinja::Error::new(
            ErrorKind::InvalidOperation,
            "Failed to read included template file",
        )
        .with_source(e)),
    });

    let mut hooks = HashSet::new();

    let ctx = jinja_context(theme);
    for template in templates.values() {
        let source = template.source.to_string_lossy();

        let builder = env.get_template(&source)?;
        let built = builder.render(&ctx)?;

        if let Some(parent) = template.target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&template.target, built)?;

        if let Some(hook) = &template.hook {
            hooks.insert(hook.clone());
        }
    }

    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
    for hook in hooks {
        run_hook(&hook, &home_dir);
    }

    Ok(())
}

/// Gets the jinja context with all the [`Theme`] colors in it.
///
/// This is used when building the templates in
/// [`build_templates`] and
/// [`Template::build`](crate::template::Template::build).
pub fn jinja_context(theme: Theme) -> Value {
    context! {
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
    }
}
