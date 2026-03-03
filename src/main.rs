use std::process::ExitCode;

use pareg::Pareg;
use termal::{eprintcln, printc, printcln};

use crate::{args::Args, config::Config, error::Error, extractor::Extractor};

pub mod args;
pub mod config;
pub mod error;
pub mod extractor;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintcln!("{'r}Error:{'_} {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse(Pareg::args())?;

    let config = match args.config {
        Some(path) => Config::from_json(path)?,
        None => Config::from_default_json(),
    };

    let Some(img) = args.img else {
        if args.should_quit {
            return Ok(());
        }
        return Err("invalid usage. Type 'gecol -h' to display help.".into());
    };

    match Extractor::extract(img, &config)? {
        Some((r, g, b)) => {
            printc!("Detected color: {'bold}#{r:02x}{g:02x}{b:02x}{'_bold} ");
            printcln!("\x1b[48;2;{r};{g};{b}m  \x1b[0m");
        }
        None => println!("No accent color detected..."),
    }

    Ok(())
}
