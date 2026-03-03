use crate::{config::Config, extractor::Extractor};

pub mod config;
pub mod error;
pub mod extractor;

fn main() {
    let image_path =
        "/mnt/hdd/Images/Wallpapers/Moutain & Painfung/wallhaven-p9dp7p.jpg";

    let config = Config::from_default_json();
    match Extractor::extract(image_path, &config) {
        Ok(Some(col)) => println!(
            "Detected color: #{:02x}{:02x}{:02x}",
            col.0, col.1, col.2
        ),
        Ok(None) => println!("No accent color detected..."),
        Err(e) => eprintln!("Error: {e}"),
    }
}
