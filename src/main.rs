use std::{
    fs::create_dir_all,
    process::{Command, ExitCode},
};

use pareg::Pareg;
use termal::eprintcln;

use crate::{
    args::{action::Action, args_struct::Args, extract::Extract},
    config::Config,
    error::Error,
    extract::extractor::Extractor,
    template::template::Template,
    theme::Theme,
};

pub mod args;
pub mod config;
pub mod error;
pub mod extract;
pub mod template;
pub mod theme;

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
    match &args.action {
        Some(Action::Extract(ext)) => extract(&args, ext),
        Some(Action::Config) => config(),
        None if args.should_quit => Ok(()),
        _ => Err("invalid usage. Type 'gecol -h' to display help.".into()),
    }
}

fn extract(args: &Args, extract: &Extract) -> Result<(), Error> {
    let Some(img) = &extract.img else {
        if args.should_quit {
            return Ok(());
        }
        return Err("invalid usage. Type 'gecol -h' to display help.".into());
    };

    let config = match &extract.config {
        Some(path) => Config::from_json(path)?,
        None => Config::from_default_json(),
    };

    match Extractor::extract(img, &config)? {
        Some(rgb) => {
            let theme = Theme::dark(rgb);
            println!("{theme}");

            let template = Template::new("test", "output");
            template.build(&theme)?;
        }
        None => println!("No accent color detected..."),
    }

    Ok(())
}

fn config() -> Result<(), Error> {
    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    create_dir_all(Config::dir())?;
    let file = Config::file();
    if !file.exists() {
        Config::default().to_default_json()?;
    }

    Command::new(editor).arg(file).spawn()?.wait()?;
    Ok(())
}
