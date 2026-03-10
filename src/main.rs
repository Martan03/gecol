use std::{
    fs::create_dir_all,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, ExitCode},
    time::Duration,
};

use gecol_core::{
    Cache, Config,
    extract::Extractor,
    template::build_templates,
    theme::{Color, Theme},
};
use indicatif::{ProgressBar, ProgressStyle};
use pareg::Pareg;
use termal::eprintcln;

use crate::{
    args::{action::Action, args_struct::Args, extract::Extract, run::Run},
    error::Error,
};

pub mod args;
pub mod error;

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
        Some(Action::Run(ext)) => run_action(&args, ext),
        Some(Action::Extract(ext)) => extract(&args, ext),
        Some(Action::Config(conf)) => config(&args, conf),
        Some(Action::ClearCache) => clear_cache(&args),
        None if args.should_quit => Ok(()),
        _ => Err("invalid usage. Type 'gecol -h' to display help.".into()),
    }
}

fn run_action(args: &Args, run: &Run) -> Result<(), Error> {
    extract_fn(args, &run.img, |conf, col| build(args, run, conf, col))
}

fn extract(args: &Args, extract: &Extract) -> Result<(), Error> {
    extract_fn(args, &extract.img, |_, color| {
        let color: Color = color.into();
        if !args.quiet {
            print!("{}  \x1b[0m ", color);
        }
        println!("{}", color.hex());
        Ok(())
    })
}

fn config(args: &Args, _conf: &args::config::Config) -> Result<(), Error> {
    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    let file = args.config.to_owned().unwrap_or_else(Config::file);

    if let Some(parent) = file.parent() {
        create_dir_all(parent)?;
    }
    if !file.exists() {
        Config::default().save(&file)?;
    }

    Command::new(editor).arg(file).spawn()?.wait()?;
    Ok(())
}

fn clear_cache(args: &Args) -> Result<(), Error> {
    let config = load_config(&args.config)?;

    let cache_path = config.cache_dir.unwrap_or_else(Cache::file);
    let msg = match std::fs::remove_file(&cache_path) {
        Ok(_) => "Cache cleared successfully!",
        Err(e) if e.kind() == ErrorKind::NotFound => {
            "Cache was already empty."
        }
        Err(e) => return Err(e.into()),
    };

    if !args.quiet {
        println!("{msg}");
    }
    Ok(())
}

fn extract_fn<F>(
    args: &Args,
    img: &Option<PathBuf>,
    color_fn: F,
) -> Result<(), Error>
where
    F: Fn(Config, (u8, u8, u8)) -> Result<(), Error>,
{
    let Some(img) = img else {
        return Err("invalid usage. Type 'gecol -h' to display help.".into());
    };

    let mut config = load_config(&args.config)?;
    config.no_cache = config.no_cache || args.no_cache;

    let spinner = get_spinner(args.quiet);
    let res = Extractor::extract_with_progress(img, &config, &spinner)?;

    if !args.quiet {
        println!();
    }

    let Some(color) = res else {
        return Err(
            "Failed to extract sufficient color and no fallback color set"
                .into(),
        );
    };
    color_fn(config, color)?;
    Ok(())
}

fn build(
    args: &Args,
    ext: &Run,
    mut conf: Config,
    color: (u8, u8, u8),
) -> Result<(), Error> {
    let spinner = get_spinner(args.quiet);

    spinner.set_message("Generating theme...");
    let theme = Theme::dark(color);
    let theme_str = format!("{theme}");

    spinner.set_message("Building templates...");
    if !ext.templates.is_empty() {
        conf.templates
            .retain(|name, _| ext.templates.contains(name));
        if conf.templates.is_empty() {
            spinner
                .finish_with_message("No matching templates found in config!");
            return Ok(());
        }
    }
    build_templates(&conf.templates, theme)?;

    spinner.finish_with_message("Templates build!");
    if !args.quiet {
        println!("\n{theme_str}");
    }
    Ok(())
}

fn load_config(path: &Option<PathBuf>) -> Result<Config, Error> {
    match path {
        Some(path) => Config::load(path).map_err(Into::into),
        None => Ok(Config::load_default()),
    }
}

fn get_spinner(quiet: bool) -> ProgressBar {
    if quiet {
        return ProgressBar::hidden();
    }

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&[
                "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✓",
            ]),
    );
    spinner
}
