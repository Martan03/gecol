use std::path::PathBuf;

use clap::Parser;
use termal::printcln;

use crate::args::action::Action;

#[derive(Debug, Clone, Default, Parser)]
// #[command(disable_help_flag = true, disable_version_flag = true)]
pub struct Args {
    #[command(subcommand)]
    pub action: Option<Action>,
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
    #[arg(short, long, global = true)]
    pub quiet: bool,
    #[arg(long, global = true)]
    pub no_cache: bool,
    // #[arg(short, long)]
    // pub help: bool,
    // #[arg(short, long)]
    // pub version: bool,
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
{'bl}Version {}{'_}

A perception-aware accent color extractor and dynamic theme generator.

{'g}Usage{'_}:
  {'c}gecol{'_} <action> [{'y}options{'_}]

{'g}Actions{'_}:
  {'y}run{'_} <image> [run options] [options]
    Runs the color extractor on the given image and builds the templates.

  {'y}extract{'_} <image> [options]
    Extracts the color from the given image.

  {'y}build{'_} <color> [run options] [options]
    Builds the templates with given color.

  {'y}config{'_} [config options] [options]
    Opens the configuration file.

  {'y}clear-cache{'_} [options]

{'g}Run options{'_}:
  {'y}-t  --template{'_} <name>:
    Builds only the given template. Can be used multiple times.

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
