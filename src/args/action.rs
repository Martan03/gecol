use crate::args::{config::Config, extract::Extract, run::Run};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Run(Run),
    Extract(Extract),
    Config(Config),
}
