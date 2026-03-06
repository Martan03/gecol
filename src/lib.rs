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
//! ### Color extraction
//!
//! To extract the accent color from an image, you can use
//! [`Extractor::extract`](crate::extract::Extractor::extract).
//!
//! ```rust
//! use gecol::{extract::Extractor, config::Config};
//!
//! match Extractor::extract("/path/to/img.jpg", &Config::default()) {
//!     Ok(Some((r, g, b))) => println!("Extracted color: rgb({r}, {g}, {b})"),
//!     _ => println!("No sufficient color found"),
//! }
//! ```
//!
//! ### Theme generating
//!
//! You can also use the [`Theme`](crate::theme::Theme) struct to generate
//! the theme using the extracted (or any) color.
//!
//! ```rust
//! use gecol::theme::Theme;
//!
//! let color = (58, 203, 175);
//! let theme = Theme::dark(color);
//! ```
//!
//! ## Links
//!
//! - **Author:** [Martan03](https://github.com/Martan03)
//! - **GitHub repository:** [gecol](https://github.com/Martan03/gecol)
//! - **Package**: [crates.io](https://crates.io/crates/gecol)
//! - **Documentation**: [docs.rs](https://docs.rs/gecol/latest/gecol/)
//! - **Author website:** [martan03.github.io](https://martan03.github.io)

pub mod config;
pub mod error;
pub mod extract;
pub mod template;
pub mod theme;
