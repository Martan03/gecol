use std::{
    fs::create_dir_all,
    process::{Command, ExitCode},
    time::Duration,
};

use gecol::{
    Config, Error,
    extract::Extractor,
    template::build_templates,
    theme::{Color, Theme},
};
use indicatif::{ProgressBar, ProgressStyle};
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
        Some(Action::Run(ext)) => run_action(&args, ext),
        Some(Action::Extract(ext)) => extract(&args, ext),
        Some(Action::Config(conf)) => config(&args, conf),
        None if args.should_quit => Ok(()),
        _ => Err("invalid usage. Type 'gecol -h' to display help.".into()),
    }
}

fn run_action(args: &Args, extract: &Extract) -> Result<(), Error> {
    extract_fn(args, extract, build)
}

fn extract(args: &Args, extract: &Extract) -> Result<(), Error> {
    extract_fn(args, extract, |_, _, color| {
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

fn extract_fn<F>(args: &Args, ext: &Extract, color_fn: F) -> Result<(), Error>
where
    F: Fn(&Args, Config, (u8, u8, u8)) -> Result<(), Error>,
{
    let Some(img) = &ext.img else {
        return Err("invalid usage. Type 'gecol -h' to display help.".into());
    };

    let config = match &args.config {
        Some(path) => Config::load(path)?,
        None => Config::load_default(),
    };

    let spinner = get_spinner(args.quiet);
    let res = Extractor::extract_with_progress(img, &config, &spinner)?;
    if !args.quiet {
        println!();
    }

    if let Some(color) = res {
        color_fn(args, config, color)?;
    }
    Ok(())
}

fn build(args: &Args, conf: Config, color: (u8, u8, u8)) -> Result<(), Error> {
    let spinner = get_spinner(args.quiet);

    spinner.set_message("Generating theme...");
    let theme = Theme::dark(color);
    let theme_str = format!("{theme}");

    spinner.set_message("Building templates...");
    build_templates(&conf.templates, theme)?;

    spinner.finish_with_message("Templates build!");
    if !args.quiet {
        println!("\n{theme_str}");
    }
    Ok(())
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
