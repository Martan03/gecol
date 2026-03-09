# gecol-core

A perception-aware accent color extractor and dynamic theme generator.

## Table of Contents

- [How to get it](#how-to-get-it)
    - [With cargo](#with-cargo)
- [Example](#example)
    - [Full pipeline](#full-pipeline)
    - [Template syntax](#template-syntax)
- [Configuration](#configuration)
- [Links](#links)

## How to get it

This crate is available on [crates.io](https://crates.io/crates/gecol-core).

### With cargo

```bash
cargo add gecol-core
```

## Example

### Full pipeline

You can extract a color, generate a theme and build a template using only
a few lines of code:

```rust
use gecol_core::prelude::*;

let config = Config::default();

// 1. Extract the color from the given image
if let Some(color) = Extractor::extract("/path/to/img.jpg", &config)? {
    // 2. Generate theme based on that color
    let theme = Theme::dark(color);

    // 3. Build the configuration file
    let template = Template::new("config.toml.template", "config.toml");
    template.build(&theme)?;

    // Or when having multiple templates (more efficient)
    let templates: Vec<Template> = get_templates();
    build_templates(&templates, theme)?;
}
```

### Template syntax

In the templates, you have access to a rich object-oriented color API:

```text
background = "{{ background }}"
transparent_bg = "{{ background.hexa(0.8) }}"
hover_color = "{{ background.lighten(0.1) }}"
border = rgba({{ primary.rgb }}aa)
```

## Configuration

The `Config` struct allows fine-tuning of the extraction algorithm, such
as saliency bonus, warmth bias and so on. You can read more about all the
fine-tuning options in the `Config` documentation.

The `Config` also contains the templates configuration. For each
template, you specify the `source` path (path to the template file) and the
`target` path (built template destination).

If the `source` is not absolute path, it automatically searches in the
`templates` directory, which by default is in `~/.config/gecol/templates`
on linux. The `target` uses home directory when the path is not absolute.

You can add a template to the configuration like this (example with waybar
colors):

```toml
[templates.waybar-colors]
source = "waybar-colors.css"
target = "~/.config/waybar/colors.css"
hook = "pkill -SIGUSR2 waybar"
```

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [gecol](https://github.com/Martan03/gecol)
- **Package**: [crates.io](https://crates.io/crates/gecol-core)
- **Documentation**: [docs.rs](https://docs.rs/gecol-core/latest/gecol/gecol-core)
- **Author website:** [martan03.github.io](https://martan03.github.io)
