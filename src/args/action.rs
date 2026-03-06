use crate::args::{config::Config, extract::Extract};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Extract(Extract),
    Config(Config),
}
