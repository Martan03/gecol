# gecol

A perception-aware accent color extractor and dynamic theme generator.

## Table of Contents

- [Installation](#installation)
    - [Arch Linux](#arch-linux)
    - [Cargo](#cargo)
    - [Build from source](#build-from-source)
- [Rust crate](#rust-crate)
- [Usage](#usage)
- [Configuration](#configuration)
- [Templates](#templates)
- [Links](#links)

## Installation

### Arch Linux

If you have Arch Linux, you can install `gecol` from
[aur](https://aur.archlinux.org/packages/gecol). When using `yay`, you can do:

```bash
yay -S gecol
```

### Cargo

Another way to install `gecol` is via the Rust toolchain (see
[rust installation page](https://www.rust-lang.org/tools/install)). When you
have the Rust Toolchain, you can install the project from source:

```bash
cargo install gecol
```

### Build from source

You can also build it directly from source. Similar to installing via `cargo`
you need the Rust toolchain:

```bash
git clone https://github.com/Martan03/gecol
cd gecol
cargo build -r
```

The binary will be `./target/release/gecol`.

## Rust crate

If you want to use this in your project, you can use the rust crate. You can
add this crate to your project using `cargo`:

```bash
cargo add gecol-core
```

You can read more about the library usage in the crate
[README](https://github.com/Martan03/gecol/blob/master/gecol-core/README.md) or
the documentation at [docs.rs](https://docs.rs/gecol-core/latest/gecol-core/).

## Usage

`gecol` can extract a color from an image, or accept a raw hex color, to
generate a dynamic theme and build your templates (more about them in
[templates](#templates) section).

Extract from a image and build templates:

```bash
gecol run /path/to/image.jpg
```

Use a specific hex color instead of an image:

```bash
gecol run "#3acbaf"
```

You can also skip building the templates and just preview the generated theme
in your terminal:

```bash
gecol run /path/to/image.jpg --skip-build
```

You can view all available commands and options by running `gecol --help`.

## Configuration

> Note: You can check the default configuration file location by running
> `gecol config -p`.

The configuration file allows fine-tuning of the extraction algorithm, such as
saliency bonus, warmth bias and so on. You can read more about all the
fine-tuning options in the `Config` struct documentation.

The config file also contains the templates configuration. Each template
contains the `source` path (path to the template file) and the `target` path
(build template destination). You can also set a `hook`, which is run after
the template is copied, which is useful with, for example, waybar, which needs
to be restarted for the configuration changes to take effect.

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

## Templates

By default the templates folder is at `~/.config/gecol/templates` on linux if
not configured otherwise. In the templates, you have access to a rich
object-oriented color API:

```text
background = "{{ background }}"
transparent_bg = "{{ background.hexa(0.8) }}"
hover_color = "{{ background.lighten(0.1) }}"
border = rgba({{ primary.rgb }}aa)
```

You can check out my
[`gecol` configuration](https://github.com/Martan03/dotfiles/tree/main/gecol)
and take inspiration.

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [gecol](https://github.com/Martan03/gecol)
- **Package**: [crates.io](https://crates.io/crates/gecol)
- **Documentation**: [docs.rs](https://docs.rs/gecol/latest/gecol/)
- **Author website:** [martan03.github.io](https://martan03.github.io)
