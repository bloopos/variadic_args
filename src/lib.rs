#![cfg_attr(feature = "no_std", no_std)]

//! A crate for handling variadic arguments.
//!
//! The documentation is unfinished for the time being for the time being.
//!
//! Use [ArgumentsBuilder] for building arguments.
//!
//! Use [Arguments] for parsing arguments.
//!
//! [ArgumentsBuilder]: ArgumentsBuilder
//! [Arguments]: Arguments

#[cfg(no_std)]
extern crate alloc;

mod argument;
mod arguments;

pub use argument::{OwnedArgument, Argument, ArgumentKind};
pub use arguments::{Arguments, ArgumentsBuilder, MAX_ARG_COUNT};
//pub mod borrowed_arg;

#[cfg(test)]
mod tests
{
    mod owned_argument;
    mod argument;
}
