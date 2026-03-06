use gecol::error::Error;
use pareg::{ArgErrKind, ArgError, Pareg};
use termal::printcln;

use crate::args::{action::Action, extract::Extract};

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub action: Option<Action>,
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
                "run" => {
                    let extract = Extract::parse(&mut args)?;
                    parsed.action = Some(Action::Extract(extract));
                }
                "config" => parsed.action = Some(Action::Config),
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

A perception-aware accent color extractor.

{'g}Usage{'_}:
  {'c}gecol{'_} [{'y}flags{'_}]
    Behaves according to the flags.

{'g}Flags{'_}:
  {'y}-i  --image{'_}
    Image to extract the color from.

  {'y}-c  --config{'_}
    Specifies custom config path.

  {'y}-h  --help{'_}
    Displays this help.

  {'y}-v --version{'_}
    Displays the version.",
            termal::gradient("Martan03", (0, 220, 255), (175, 80, 255)),
            Self::VERSION_NUMBER
        );
    }
}
