use std::{
    fs::create_dir_all,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, ExitCode},
    time::Duration,
};

use clap::Parser;
use gecol_core::{
    Cache, Config,
    extract::Extractor,
    template::build_templates,
    theme::{Color, Theme},
};
use indicatif::{ProgressBar, ProgressStyle};
use termal::eprintcln;

use crate::{
    args::{
        action::{Action, Build, Extract, Preview, Run, parse_hex_col},
        args_struct::Args,
    },
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
    let args = Args::parse();

    // if args.version {
    //     Args::version();
    //     return Ok(());
    // }
    // if args.help || args.action.is_none() {
    //     Args::help();
    //     return Ok(());
    // }

    match args.action.as_ref().unwrap() {
        Action::Run(run_args) => handle_run(&args, run_args),
        Action::Extract(ext_args) => handle_extract(&args, ext_args),
        Action::Build(build_args) => handle_build(&args, build_args),
        Action::Preview(prev_args) => handle_preview(&args, prev_args),
        Action::List => handle_list(&args),
        Action::Config => handle_config(&args),
        Action::ClearCache => clear_cache(&args),
    }
}

fn handle_run(args: &Args, run: &Run) -> Result<(), Error> {
    extract_fn(args, &run.img, |mut conf, col| {
        conf.theme_type = run.theme.unwrap_or(conf.theme_type);
        build(args, conf, &run.templates, col)
    })
}

fn handle_extract(args: &Args, extract: &Extract) -> Result<(), Error> {
    extract_fn(args, &extract.img, |_, color| {
        let color: Color = color.into();
        if !args.quiet {
            print!("{}  \x1b[0m ", color);
        }
        println!("{}", color.hex());
        Ok(())
    })
}

fn handle_build(args: &Args, ext: &Build) -> Result<(), Error> {
    let mut config = load_config(&args.config)?;
    config.theme_type = ext.theme.unwrap_or(config.theme_type);
    build(args, config, &ext.templates, ext.color)
}

fn handle_preview(args: &Args, ext: &Preview) -> Result<(), Error> {
    let mut config = load_config(&args.config)?;
    config.theme_type = ext.theme.unwrap_or(config.theme_type);

    let color = if let Ok(color) = parse_hex_col(&ext.target) {
        color
    } else {
        let path = PathBuf::from(&ext.target);
        extract_color(args, &path, &mut config)?
    };

    let theme = Theme::generate(config.theme_type, color);
    println!("{theme}");
    Ok(())
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

fn extract_fn<F>(args: &Args, img: &PathBuf, color_fn: F) -> Result<(), Error>
where
    F: Fn(Config, (u8, u8, u8)) -> Result<(), Error>,
{
    let mut config = load_config(&args.config)?;
    let color = extract_color(args, img, &mut config)?;
    color_fn(config, color)
}

fn extract_color(
    args: &Args,
    img: &PathBuf,
    conf: &mut Config,
) -> Result<(u8, u8, u8), Error> {
    conf.no_cache = conf.no_cache || args.no_cache;

    let spinner = get_spinner(args.quiet);
    let res = Extractor::extract_with_progress(img, conf, &spinner)?;
    if !args.quiet {
        println!();
    }

    res.ok_or_else(|| {
        "Failed to extract sufficient color and no fallback color set".into()
    })
}

fn build(
    args: &Args,
    mut conf: Config,
    templates: &Vec<String>,
    color: (u8, u8, u8),
) -> Result<(), Error> {
    let spinner = get_spinner(args.quiet);

    spinner.set_message("Generating theme...");
    let theme = Theme::generate(conf.theme_type, color);
    let theme_str = format!("{theme}");

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
