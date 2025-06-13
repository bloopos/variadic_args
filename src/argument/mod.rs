mod variant_info;
mod inlined;
mod owned;
mod boxed_argument;
mod discriminant;
// Unsure what to do about it.
//mod borrowed_arg;

pub use owned::OwnedArgument;

pub(crate) use variant_info::VariantHandle;

mod argument;

pub use argument::{Argument, ArgumentKind};
