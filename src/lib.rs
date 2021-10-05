#![doc = include_str!("../README.md")]

mod builder;
mod corpus;
mod root;

pub use crate::builder::CorpusBuilder;
pub use crate::corpus::{Corpus, MaybePath};
pub use crate::root::RootLocation;

#[derive(thiserror::Error, Debug)]
pub enum CorpusError {
    #[error("There is no home directory")]
    NoHomeDir,

    #[error("Current directory does not exist or insufficient permissions")]
    InvalidCurrentDir,
}

/// Shorthand for constructing an empty builder.
pub fn builder() -> CorpusBuilder {
    CorpusBuilder::default()
}
