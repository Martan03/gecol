use crate::args::{build::Build, config::Config, extract::Extract, run::Run};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Run(Run),
    Extract(Extract),
    Build(Build),
    Config(Config),
    ClearCache,
}
