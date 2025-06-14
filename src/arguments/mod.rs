mod arguments;
mod builder;

/// The maximum amount of arguments allowed inside a arguments container.
pub const MAX_ARG_COUNT : usize = 1024;

pub use arguments::Arguments;
pub use builder::ArgumentsBuilder;
