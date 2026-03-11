use std::path::PathBuf;

use clap::Parser;
use termal::printcln;

use crate::args::action::Action;

#[derive(Debug, Clone, Default, Parser)]
#[command(disable_help_flag = true, disable_version_flag = true)]
pub struct Args {
    #[command(subcommand)]
    pub action: Option<Action>,
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
    #[arg(short, long, global = true)]
    pub quiet: bool,
    #[arg(long, global = true)]
    pub no_cache: bool,
    #[arg(short, long, exclusive = true)]
    pub help: bool,
    #[arg(short, long, exclusive = true)]
    pub version: bool,
}

impl Args {
    pub const VERSION_NUMBER: &'static str = {
        let v = option_env!("CARGO_PKG_VERSION");
        if let Some(v) = v { v } else { "unknown" }
    };

    pub fn version() {
        println!("gecol {}", Self::VERSION_NUMBER)
    }

    pub fn help() {
        printcln!(
            "Welcome to {'g}gecol{'_} by {}{'_}
{'gr}Version {}{'_}

A perception-aware accent color extractor and dynamic theme generator.

{'g}Usage{'_}:
  {'c}gecol{'_} <action> [{'y}options{'_}]

{'g}Actions{'_}:
  {'y}run{'_} <image|hex_color> [run options] [options]
    Extracts a color from an image, or uses given color, to generate a theme
    and build templates.

  {'y}list{'_} [options]
    Lists all the configured templates.

  {'y}config{'_} [config options] [options]
    Opens the configuration file.

  {'y}clear-cache{'_} [options]

{'g}Run options{'_}:
  <image|hex_color>
    The target image path or a hex color (e.g. \"#3acbaf\"). If image, runs
    the color extraction.

  {'y}-t  --template{'_} <name>
    Builds only the given template. Can be used multiple times.

  {'y}-T  --theme{'_} <light|dark>
    Overrides the theme type for this run.

  {'y}--skip-build{'_}
    Skips building the templates, prints generated theme.
    {'gr}(Conflicts with --extract-only, --template){'_}

  {'y}--extract-only{'_}
    Only extracts and prints color from the image input.
    {'gr}(Conflicts with --skip-build, --template, --theme){'_}

{'g}Config options{'_}:
  {'y}-p  --path{'_}
    Prints the default configuration file location.

{'g}Options{'_}:
  These options can be used in any of the actions.

  {'y}-c  --config{'_} <file>
    Specifies custom config path

  {'y}-q  --quiet{'_}
    Turns off unnecessary printing to terminal.

  {'y}--no-cache{'_}
    Disables using the color cache.

{'g}Flags{'_}:
  {'y}-h  --help{'_}
    Displays this help.

  {'y}-v --version{'_}
    Displays the current version.",
            termal::gradient("Martan03", (0, 220, 255), (175, 80, 255)),
            Self::VERSION_NUMBER
        );
    }
}
