use std::path::PathBuf;

use pareg::{ArgErrKind, ArgError, Pareg};
use termal::printcln;

use crate::error::Error;

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub img: Option<PathBuf>,
    pub config: Option<PathBuf>,
    pub should_quit: bool,
}

impl Args {
    pub const VERSION_NUMBER: &'static str = {
        let v = option_env!("CARGO_PKG_VERSION");
        if let Some(v) = v { v } else { "unknown" }
    };

    pub fn parse(mut args: Pareg) -> Result<Self, Error> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg {
                "-i" | "--image" => parsed.img = args.next_arg()?,
                "-c" | "--config" => parsed.config = args.next_arg()?,
                "-v" | "--version" => {
                    parsed.should_quit = true;
                    println!("gecol {}", Self::VERSION_NUMBER)
                }
                "h" | "-h" | "--help" => {
                    parsed.should_quit = true;
                    Self::help();
                }
                _ => return Self::unknown_arg(&arg),
            }
        }
        Ok(parsed)
    }

    fn unknown_arg(arg: &str) -> Result<Self, Error> {
        Err(Error::Pareg(ArgError::from_msg(
            ArgErrKind::UnknownArgument,
            "invalid argument",
            arg,
        )))
    }

    pub fn help() {
        printcln!(
            "Welcome to {'g}gecol{'_} by {}{'_}
{'bl}Version {}{'_}

A perception-aware accent color extractor.

{'g}Usage{'_}:
  {'c}gecol{'_} [{'y}flags{'_}]
    Behaves according to the flags.

{'g}Flags{'_}:
  {'y}-i  --image{'_}
    Image to extract the color from.

  {'y}-h  --help{'_}
    Displays this help.

  {'y}-v --version{'_}
    Displays the version.",
            termal::gradient("Martan03", (0, 220, 255), (175, 80, 255)),
            Self::VERSION_NUMBER
        );
    }
}
