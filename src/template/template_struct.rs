use std::{
    path::{Path, PathBuf},
    process::Command,
};

use minijinja::Environment;
use serde::{Deserialize, Serialize};

use crate::{error::Error, template::jinja_context, theme::Theme};

/// Holds the template configuration:
/// - `source` path - file path to the template.
/// - `target` path - template build destination.
///
/// # Template syntax
///
/// In the template, you have access to a rich object-oriented color API.
///
/// ## Colors in template
///
/// You have access to all colors defined in the
/// [`Theme`](crate::theme::Theme). To access a color, you can add this
/// anywhere into your template file:
///
/// ```text
/// {{ color_name }}
/// ```
///
/// Where `color_name` should be replace by the color you want to use.
/// Available colors are:
///
/// - `primary`
/// - `secondary`
/// - `background`
/// - `surface`
/// - `border`
/// - `foreground`
/// - `muted`
/// - `success`
/// - `warning`
/// - `error`
///
/// ## Color methods
///
/// The following methods are available to manipulate the color object. They
/// can be chained together.
///
/// - `lighten(amount)`: lightens the color by adding a value to it.
/// - `brighten(amount)`: brightens the color relatively by a given multiplier.
/// - `darken(amount)`: darkens the color by adding a value to it.
/// - `dim(amount)`: dims the color relatively by a given multiplier.
/// - `saturate(amount)`: increases the saturation (chroma component) of the
///   color.
/// - `desaturate(amount)`: descreases the saturation (chroma component) of the
///   color.
///
/// ## Color formats
///
/// You can also convert the color into multiple text formats. Note that
/// after a color is formatted, it becomes a string and you cannot access the
/// menipulation methods anymore. You should use these last.
///
/// - `hex`: `#rrggbb` (default if no formatter is specified)
/// - `hexa(alpha)`: `#rrggbbaa`, where `alpha` is the provided float
///   (0.0 to 1.0).
/// - `rgb`: `r,g,b` format (e.g. `42,128,56`).
/// - `rgba(alpha)`: `r,g,b,a` format (e.g. `42,128,56,0.8`), where `alpha` is
///   the provided float (0.0 to 1.0).
/// - `strip`: hex without the leading `#` character - `rrggbb`.
/// - `r`, `g`, `b`: extracts the corresponding raw RGB color component.
///
/// # Example
///
/// ```text
/// /* Defaults to hex format. */
/// bg_color = "{{ background }}"
///
/// /* Chains methods before formatting. */
/// bg_hover = "rgb({{ background.lighten(0.1).rgb }})"
///
/// /* Creates transparent color. */
/// border = "rgba({{ primary.rgba(0.8) }})"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub source: PathBuf,
    pub target: PathBuf,
    // A command to be executed after the template is built.
    //
    // Some program require restart after their configuration is edited, such
    // as waybar, so you can restart them here.
    #[serde(default)]
    pub hook: Option<String>,
}

impl Template {
    /// Creates new template with `source` as the template file path and
    /// `target` as the build destination.
    pub fn new<P1, P2>(source: P1, target: P2) -> Self
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        Self {
            source: source.as_ref().to_owned(),
            target: target.as_ref().to_owned(),
            hook: None,
        }
    }

    /// Builds the template at `source` and saves it to `target`.
    ///
    /// Note that it's more efficient to build multiple templates using
    /// [`build_templates`](crate::template::build_templates), instead of
    /// using this function on each of the templates.
    pub fn build(&self, theme: &Theme) -> Result<(), Error> {
        let content = std::fs::read_to_string(&self.source)?;

        let mut env = Environment::new();
        let source = self.source.to_string_lossy();
        env.add_template(&source, &content)?;

        let template = env.get_template(&source)?;
        let ctx = jinja_context(theme.clone());
        let built = template.render(ctx)?;

        if let Some(parent) = self.target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.target, built)?;

        self.run_hook();
        Ok(())
    }

    /// Runs the template hook if set.
    ///
    /// It runs the set hook in the home directory (or in `/` if home cannot
    /// be found).
    pub fn run_hook(&self) {
        let Some(hook) = &self.hook else {
            return;
        };

        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        run_hook(hook, &home_dir);
    }
}

/// Runs the given hook on the given directory.
pub fn run_hook(hook: &str, dir: &Path) {
    let status = Command::new("sh")
        .arg("-c")
        .arg(hook)
        .current_dir(dir)
        .status();
    if let Err(e) = status {
        eprintln!("Warning: Failed to execute hook '{hook}': {e}",);
    }
}
