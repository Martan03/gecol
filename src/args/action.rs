use crate::args::extract::Extract;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Extract(Extract),
    Config,
}
