#[cfg(no_std)]
use core::any::Any;

#[cfg(not(no_std))]
use std::any::Any;

use super::owned::OwnedArgument;

/// An encapsulated wrapper on both traits Any and Clone.
///
/// Or in short, a trait implementing both Any and Clone are not dyn compatible.
/// As such, we have to resort to a hack in order for us to get the right variable,
/// which is why we require Any first.
///
/// The next implementation acts as a generic wrapper.
pub(crate) trait VariantHandle : Any
{
    /// A wrapper for clone, returning an OwnedArgument.
    ///
    /// Internally, it clones the object first. Then, instead of returning Self,
    /// it instead returns a creates an OwnedArgument from the cloned object. All of
    /// this workload is meant to maintain dyn compatibility.
    fn clone_object(&self) -> OwnedArgument;
}

impl<T> VariantHandle for T
where
    T: Any + Clone
{
    #[inline(always)]
    fn clone_object(&self) -> OwnedArgument
    {
        OwnedArgument::new(self.clone())
    }
}
