//! A perception-aware accent color extractor and dynamic theme generator.
//!
//! ## How to get it
//!
//! This crate is available on [crates.io](https://crates.io/crates/gecol).
//!
//! ### With cargo
//!
//! ```bash
//! cargo add gecol
//! ```
//!
//! ## Example
//!
//! ### Full pipeline
//!
//! You can extract a color, generate a theme and build a template using only
//! a few lines of code:
//!
//! ```rust,no_run
//! use gecol::prelude::*;
//! # fn get_templates() -> Vec<Template> { vec![] }
//!
//! # fn main() -> Result<(), gecol::Error> {
//! let config = Config::default();
//!
//! // 1. Extract the color from the given image
//! if let Some(color) = Extractor::extract("/path/to/img.jpg", &config)? {
//!     // 2. Generate theme based on that color
//!     let theme = Theme::dark(color);
//!
//!     // 3. Build the configuration file
//!     let template = Template::new("config.toml.template", "config.toml");
//!     template.build(&theme)?;
//!
//!     // Or when having multiple templates (more efficient)
//!     let templates: Vec<Template> = get_templates();
//!     build_templates(&templates, theme)?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Template syntax
//!
//! In the templates, you have access to a rich object-oriented color API:
//!
//! ```text
//! background = "{{ background }}"
//! transparent_bg = "{{ background.hexa(0.8) }}"
//! hover_color = "{{ background.lighten(0.1) }}"
//! border = rgba({{ primary.rgb }}aa)
//! ```
//!
//! ## Configuration
//!
//! The [`Config`](crate::Config) struct allows fine-tuning of the extraction
//! algorithm, such as saliency bonus, warmth bias and so on. You can read
//! more about all the fine-tuning options in the [`Config`](crate::Config)
//! documentation.
//!
//! The [`Config`](crate::Config) also contains the templates configuration.
//! For each template, you specify the `source` path (path to the template
//! file) and the `target` path (built template destination).
//!
//! If the `source` is not absolute path, it automatically searches in the
//! `templates` directory, which by default is in `~/.config/gecol/templates`
//! on linux. The `target` uses home directory when the path is not absolute.
//!
//! You can add a template to the configuration like this:
//!
//! ```toml
//! [[template]]
//! source = "some-config.json.template"
//! target = "/home/user/.config/some-app/some-config.json"
//! ```
//!
//! ## Links
//!
//! - **Author:** [Martan03](https://github.com/Martan03)
//! - **GitHub repository:** [gecol](https://github.com/Martan03/gecol)
//! - **Package**: [crates.io](https://crates.io/crates/gecol)
//! - **Documentation**: [docs.rs](https://docs.rs/gecol/latest/gecol/)
//! - **Author website:** [martan03.github.io](https://martan03.github.io)

mod config;
mod error;
pub mod extract;
pub mod prelude;
pub mod template;
pub mod theme;

pub use config::Config;
pub use error::Error;
