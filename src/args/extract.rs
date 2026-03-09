use std::path::PathBuf;

use pareg::Pareg;

use crate::{args::args_struct::Args, error::Error};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Extract {
    pub img: Option<PathBuf>,
}

impl Extract {
    pub fn parse(args: &mut Pareg, pargs: &mut Args) -> Result<Self, Error> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg {
                arg if arg.starts_with('-') => pargs.shared_flags(args)?,
                img => parsed.img = Some(PathBuf::from(img)),
            }
        }
        Ok(parsed)
    }
}
