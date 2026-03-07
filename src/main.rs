use std::{
    fs::create_dir_all,
    process::{Command, ExitCode},
};

use gecol::{
    Config, Error,
    extract::Extractor,
    template::build_templates,
    theme::{Color, Theme},
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
    if args.should_quit {
        return Ok(());
    }

    match &args.action {
        Some(Action::Run(ext)) => run_action(ext),
        Some(Action::Extract(ext)) => extract(ext),
        Some(Action::Config(conf)) => config(conf),
        None if args.should_quit => Ok(()),
        _ => Err("invalid usage. Type 'gecol -h' to display help.".into()),
    }
}

fn run_action(extract: &Extract) -> Result<(), Error> {
    extract_fn(extract, build)
}

fn extract(extract: &Extract) -> Result<(), Error> {
    extract_fn(extract, |_, color| {
        let color: Color = color.into();
        println!("{}  \x1b[0m {}", color, color.hex());
        Ok(())
    })
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

fn extract_fn<F>(extract: &Extract, color_fn: F) -> Result<(), Error>
where
    F: Fn(Config, (u8, u8, u8)) -> Result<(), Error>,
{
    let Some(img) = &extract.img else {
        return Err("invalid usage. Type 'gecol -h' to display help.".into());
    };

    let config = match &extract.config {
        Some(path) => Config::load(path)?,
        None => Config::load_default(),
    };

    if let Some(color) = Extractor::extract(img, &config)? {
        color_fn(config, color)?;
    }
    Ok(())
}

fn build(config: Config, color: (u8, u8, u8)) -> Result<(), Error> {
    let theme = Theme::dark(color);
    let theme_str = format!("{theme}");

    build_templates(&config.templates, theme)?;

    println!("{theme_str}");
    Ok(())
}
