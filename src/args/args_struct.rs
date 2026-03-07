use std::path::PathBuf;

use gecol::Error;
use pareg::{ArgErrKind, ArgError, Pareg};
use termal::printcln;

use crate::args::{action::Action, config::Config, extract::Extract};

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub action: Option<Action>,
    pub config: Option<PathBuf>,
    pub quiet: bool,
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
                    let extract = Extract::parse(&mut args, &mut parsed)?;
                    parsed.action = Some(Action::Run(extract));
                }
                "extract" => {
                    let extract = Extract::parse(&mut args, &mut parsed)?;
                    parsed.action = Some(Action::Extract(extract));
                }
                "config" => {
                    let config = Config::parse(&mut args, &mut parsed)?;
                    parsed.action = Some(Action::Config(config));
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

    /// Handles the shared flags across actions.
    pub fn shared_flags(&mut self, args: &mut Pareg) -> Result<(), Error> {
        let Some(arg) = args.cur() else { return Ok(()) };
        match arg {
            "-c" | "--config" => self.config = args.next_arg()?,
            "-q" | "--quiet" => self.quiet = true,
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
  {'c}gecol{'_} [action] [{'y}options{'_}]

{'g}Actions{'_}:
  {'y}run{'_} <IMAGE> [run options]
    Runs the color extractor on the given image and builds the templates.

  {'y}extract{'_} <IMAGE> [run options]
    Extracts the color from the given image.

  {'y}config{'_} [config options]
    Opens the configuration file.

{'g}Run options{'_}:
  {'y}-c  --config{'_} <FILE>
    Specifies custom config path.

{'g}Config options{'_}:
  {'y}-c  --config{'_} <FILE>
    Specifies custom config path.

  {'y}-p  --path{'_}
    Prints the default configuration file location.

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
