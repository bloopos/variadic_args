mod arguments;
mod builder;

pub use arguments::Arguments;
pub use builder::ArgumentsBuilder;

/// The maximum amount of arguments allowed inside a arguments container.
pub const MAX_ARG_COUNT : usize = 1024;
