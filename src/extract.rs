use std::path::Path;

use gecol_core::{extract::ExtractStep, prelude::Extractor};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    args::args_struct::Args, config::Config, error::Error, get_spinner,
};

pub fn extract_color(
    args: &Args,
    config: &mut Config,
    img: &Path,
) -> Result<(u8, u8, u8), Error> {
    config.no_cache = config.no_cache || args.no_cache;
    let spinner = get_spinner(args.quiet);
    let set_spinner = |step: ExtractStep| {
        if step.is_final() {
            spinner.finish_with_message(step.to_string());
        } else {
            spinner.set_message(step.to_string());
        }
    };

    let conf = &config.extraction;
    let color = if config.no_cache {
        Extractor::extract_with_progress(img, conf, set_spinner)?
    } else {
        Extractor::extract_cached_with_progress(img, conf, None, set_spinner)?
    };

    let color = color.or_else(|| config.fallback_color());
    if !color.is_some() {
        spinner_err(&spinner, "Failed to extract sufficient color.");
    }

    if !args.quiet {
        println!();
    }

    color.ok_or_else(|| {
        "Failed to extract sufficient color and no fallback color set".into()
    })
}

fn spinner_err(spinner: &ProgressBar, msg: &str) {
    spinner.set_style(
        ProgressStyle::with_template("{prefix:.red} {msg}").unwrap(),
    );
    spinner.set_prefix("✗");
    spinner.abandon_with_message(msg.to_owned());
}
