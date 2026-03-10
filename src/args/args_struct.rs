use std::path::PathBuf;

use pareg::{ArgErrKind, ArgError, Pareg};
use termal::printcln;

use crate::{
    args::{
        action::Action, build::Build, config::Config, extract::Extract,
        run::Run,
    },
    error::Error,
};

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub action: Option<Action>,
    pub config: Option<PathBuf>,
    pub quiet: bool,
    pub no_cache: bool,
    pub should_quit: bool,
}

impl Args {
    pub const VERSION_NUMBER: &'static str = {
        let v = option_env!("CARGO_PKG_VERSION");
        if let Some(v) = v { v } else { "unknown" }
    };

    pub fn parse(mut args: Pareg) -> Result<Self, Error> {
        let mut parsed = Self::default();
        if let Some(arg) = args.next() {
            match arg {
                "run" => {
                    let run = Run::parse(&mut args, &mut parsed)?;
                    parsed.action = Some(Action::Run(run));
                }
                "extract" => {
                    let extract = Extract::parse(&mut args, &mut parsed)?;
                    parsed.action = Some(Action::Extract(extract));
                }
                "build" => {
                    let build = Build::parse(&mut args, &mut parsed)?;
                    parsed.action = Some(Action::Build(build));
                }
                "config" => {
                    let config = Config::parse(&mut args, &mut parsed)?;
                    parsed.action = Some(Action::Config(config));
                }
                "clear-cache" => {
                    parsed.parse_generic(&mut args)?;
                    parsed.action = Some(Action::ClearCache);
                }
                "-v" | "--version" => {
                    parsed.should_quit = true;
                    println!("gecol {}", Self::VERSION_NUMBER)
                }
                "h" | "-h" | "--help" => {
                    parsed.should_quit = true;
                    Self::help();
                }
                _ => return Err(Self::unknown_arg(arg)),
            }
        }
        Ok(parsed)
    }

    pub fn parse_generic(&mut self, args: &mut Pareg) -> Result<(), Error> {
        while let Some(arg) = args.next() {
            match arg {
                _ => self.shared_flags(args)?,
            }
        }
        Ok(())
    }

    /// Handles the shared flags across actions.
    pub fn shared_flags(&mut self, args: &mut Pareg) -> Result<(), Error> {
        let Some(arg) = args.cur() else { return Ok(()) };
        match arg {
            "-c" | "--config" => self.config = args.next_arg()?,
            "-q" | "--quiet" => self.quiet = true,
            "--no-cache" => self.no_cache = true,
            arg => return Err(Self::unknown_arg(arg)),
        }
        Ok(())
    }

    pub fn unknown_arg(arg: &str) -> Error {
        Error::Pareg(ArgError::from_msg(
            ArgErrKind::UnknownArgument,
            "invalid argument",
            arg,
        ))
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
