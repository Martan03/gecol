use std::{
    fs::create_dir_all,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, ExitCode},
    time::Duration,
};

use clap::Parser;
use gecol_core::{
    Cache,
    template::build_templates,
    theme::{Color, Theme},
};
use indicatif::{ProgressBar, ProgressStyle};
use termal::{eprintcln, formatc};

use crate::{
    args::{
        action::{Action, Run, parse_hex_col},
        args_struct::Args,
    },
    config::Config,
    error::Error,
    extract::extract_color,
};

pub mod args;
pub mod config;
pub mod error;
pub mod extract;

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
    let args = Args::parse();

    if args.version {
        Args::version();
        return Ok(());
    }
    if args.help || args.action.is_none() {
        Args::help();
        return Ok(());
    }

    match args.action.as_ref().unwrap() {
        Action::Run(run_args) => handle_run(&args, run_args),
        Action::List => handle_list(&args),
        Action::Config => handle_config(&args),
        Action::ClearCache => clear_cache(&args),
    }
}

fn handle_run(args: &Args, run: &Run) -> Result<(), Error> {
    let mut config = load_config(&args.config)?;
    config.theme_type = run.theme.unwrap_or(config.theme_type);

    let (color, color_input) = if let Ok(color) = parse_hex_col(&run.target) {
        (color, true)
    } else {
        let path = PathBuf::from(&run.target);
        (extract_color(args, &mut config, &path)?, false)
    };

    if run.extract_only {
        if color_input {
            return Err(formatc!(
                "'{'dy}--extract-only{'_}' expects image as input, color given."
            )
            .into());
        }
        extract_only(args, color);
        return Ok(());
    }

    let theme = Theme::generate(config.theme_type, color);
    let theme_str = theme.to_string();
    if !run.skip_build {
        build(args, config, &run.templates, theme)?;
    }

    if run.skip_build || !args.quiet {
        println!("{theme_str}");
    }

    Ok(())
}

fn extract_only(args: &Args, color: (u8, u8, u8)) {
    let color: Color = color.into();
    if !args.quiet {
        print!("{}  \x1b[0m ", color);
    }
    println!("{}", color.hex());
}

fn handle_list(args: &Args) -> Result<(), Error> {
    let config = load_config(&args.config)?;
    if config.templates.is_empty() {
        println!("No templates found in you configuration file.");
        return Ok(());
    }

    let mut keys: Vec<&String> = config.templates.keys().collect();
    keys.sort();

    println!("Available templates:");
    for template in keys {
        println!(" - {template}");
    }
    Ok(())
}

fn handle_config(args: &Args) -> Result<(), Error> {
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

fn build(
    args: &Args,
    mut conf: Config,
    templates: &[String],
    theme: Theme,
) -> Result<(), Error> {
    let spinner = get_spinner(args.quiet);

    spinner.set_message("Building templates...");
    if !templates.is_empty() {
        conf.templates.retain(|name, _| templates.contains(name));
        if conf.templates.is_empty() {
            spinner
                .finish_with_message("No matching templates found in config!");
            return Ok(());
        }
    }
    build_templates(&conf.templates, theme)?;

    spinner.finish_with_message("Templates build!");
    if !args.quiet {
        println!();
    }
    Ok(())
}

fn load_config(path: &Option<PathBuf>) -> Result<Config, Error> {
    match path {
        Some(path) => Config::load(path),
        None => Ok(Config::load_default()),
    }
}

pub fn get_spinner(quiet: bool) -> ProgressBar {
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
