use gecol::Error;
use pareg::Pareg;

use crate::args::args_struct::Args;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Config;

impl Config {
    pub fn parse(args: &mut Pareg, pargs: &mut Args) -> Result<Self, Error> {
        let parsed = Self;
        while let Some(arg) = args.next() {
            match arg {
                "-p" | "--path" => {
                    println!("{}", gecol::Config::file().to_string_lossy());
                    pargs.should_quit = true;
                    return Ok(parsed);
                }
                _ => pargs.shared_flags(args)?,
            }
        }
        Ok(parsed)
    }
}
