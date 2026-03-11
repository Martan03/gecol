mod config;
mod extractor;
mod scores;

use std::fmt::Display;

pub use config::ExtractionConfig;
pub use extractor::Extractor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtractStep {
    CheckingCache,
    FinishedWithCache,

    OpeningImage,
    ResizingImage,
    ExtractingColors,
    Clustering,
    Finished,
}

impl ExtractStep {
    /// Checks if the current step is the final step.
    pub fn is_final(&self) -> bool {
        matches!(self, ExtractStep::Finished)
            || matches!(self, ExtractStep::FinishedWithCache)
    }
}

impl Display for ExtractStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractStep::CheckingCache => write!(f, "Checking cache..."),
            ExtractStep::FinishedWithCache => {
                write!(f, "Color loaded from cache!")
            }

            ExtractStep::OpeningImage => write!(f, "Opening image..."),
            ExtractStep::ResizingImage => write!(f, "Resizing image..."),
            ExtractStep::ExtractingColors => {
                write!(f, "Extracting perception-aware colors...")
            }
            ExtractStep::Clustering => {
                write!(f, "Running K-Means clustering...")
            }
            ExtractStep::Finished => write!(f, "Color extracted!"),
        }
    }
}
