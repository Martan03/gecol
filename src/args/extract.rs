use std::path::PathBuf;

use gecol::Error;
use pareg::Pareg;

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
                "-c" | "--config" => parsed.config = args.next_arg()?,
                img => parsed.img = Some(PathBuf::from(img)),
            }
        }
        Ok(parsed)
    }
}
