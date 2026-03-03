use std::path::PathBuf;

use pareg::Pareg;

use crate::{args::args_struct::Args, error::Error};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Extract {
    pub img: Option<PathBuf>,
    pub config: Option<PathBuf>,
}

impl Extract {
    pub fn parse(args: &mut Pareg) -> Result<Self, Error> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg {
                "-i" | "--image" => parsed.img = args.next_arg()?,
                "-c" | "--config" => parsed.config = args.next_arg()?,
                "--" => break,
                _ => return Err(Args::unknown_arg(&arg)),
            }
        }
        Ok(parsed)
    }
}
