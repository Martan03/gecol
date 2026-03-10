use crate::error::Error;
use pareg::Pareg;

use crate::args::args_struct::Args;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Build {
    pub color: Option<(u8, u8, u8)>,
    pub templates: Vec<String>,
}

impl Build {
    pub fn parse(args: &mut Pareg, pargs: &mut Args) -> Result<Self, Error> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg {
                "-t" | "--template" => parsed.templates.push(args.next_arg()?),
                arg if arg.starts_with('-') => pargs.shared_flags(args)?,
                col => parsed.color = Some(Self::parse_col(col)?),
            }
        }
        Ok(parsed)
    }

    fn parse_col(val: &str) -> Result<(u8, u8, u8), Error> {
        let hex = val.trim_start_matches('#');
        if hex.len() == 6
            && let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            )
        {
            return Ok((r, g, b));
        }

        Err(Error::Pareg(pareg::ArgError::invalid_value(
            "Color is expected in format '#rrggbb'.",
            val,
        )))
    }
}
