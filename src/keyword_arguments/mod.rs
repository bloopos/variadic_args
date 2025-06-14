use smartstring::{SmartString, LazyCompact};

mod keyed_args;
mod error;
mod builder;

pub type Key = SmartString<LazyCompact>;

pub use error::{Error, ErrorKind};

pub use keyed_args::KeywordArguments;

pub use builder::KeywordArgumentsBuilder;
