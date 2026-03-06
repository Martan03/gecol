use std::{
    fs::create_dir_all,
    process::{Command, ExitCode},
};

use gecol::{
    Config, Error, extract::Extractor, template::build_templates, theme::Theme,
};
use pareg::Pareg;
use termal::eprintcln;

use crate::args::{action::Action, args_struct::Args, extract::Extract};

pub mod args;

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
        Some(Action::Config(conf)) => config(conf),
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
        Some(path) => Config::load(path)?,
        None => Config::load_default(),
    };

    match Extractor::extract(img, &config)? {
        Some(rgb) => {
            let theme = Theme::dark(rgb);
            println!("{theme}");

            build_templates(&config.templates, theme)?;
        }
        None => println!("No accent color detected..."),
    }

    Ok(())
}

fn config(conf: &args::config::Config) -> Result<(), Error> {
    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    let file = conf.path.to_owned().unwrap_or_else(Config::file);

    if let Some(parent) = file.parent() {
        create_dir_all(parent)?;
    }
    if !file.exists() {
        Config::default().save(&file)?;
    }

    Command::new(editor).arg(file).spawn()?.wait()?;
    Ok(())
}
