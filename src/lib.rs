#![cfg_attr(feature = "no_std", no_std)]

#[cfg(no_std)]
extern crate alloc;

mod argument;
mod arguments;

pub use argument::{OwnedArgument, Argument, ArgumentKind};
pub use arguments::Arguments;
//pub mod borrowed_arg;

#[cfg(test)]
mod tests
{
    mod owned_argument;
}
