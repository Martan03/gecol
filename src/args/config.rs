use std::path::PathBuf;

use gecol::error::Error;
use pareg::Pareg;

use crate::args::args_struct::Args;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Config {
    pub path: Option<PathBuf>,
}

impl Config {
    pub fn parse(args: &mut Pareg) -> Result<Self, Error> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg {
                "-p" | "--path" => parsed.path = args.next_arg()?,
                "--" => break,
                _ => return Err(Args::unknown_arg(arg)),
            }
        }
        Ok(parsed)
    }
}
