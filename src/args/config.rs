use std::path::PathBuf;

use gecol::Error;
use pareg::Pareg;

use crate::args::args_struct::Args;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Config {
    pub path: Option<PathBuf>,
}

impl Config {
    pub fn parse(args: &mut Pareg, pargs: &mut Args) -> Result<Self, Error> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg {
                "-c" | "--config" => parsed.path = args.next_arg()?,
                "-p" | "--path" => {
                    println!("{}", gecol::Config::file().to_string_lossy());
                    pargs.should_quit = true;
                    return Ok(parsed);
                }
                _ => return Err(Args::unknown_arg(arg)),
            }
        }
        Ok(parsed)
    }
}
