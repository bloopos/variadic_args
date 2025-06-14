#[cfg(no_std)]
use core::any::Any;

#[cfg(not(no_std))]
use std::any::Any;

use super::owned::OwnedArgument;

/// An encapsulated trait that provides information about the pointer itself.
///
/// This acts as a rather, unsafe trait. The only reason why it is not marked unsafe
/// in the first place is due to how it gets handled throughout the code.
///
/// # Safety
/// The only safety rule to follow is that both metadata and raw_pointer guarantee
/// that the data points to a valid dyn object.
pub(super) unsafe trait PointerInfo
{
    /// Provides metadata information about the pointer itself.
    ///
    /// Due to how it works, the metadata is usually stored inside the pointer itself.
    /// As such, this trait guarantees we get the right pointer.
    ///
    /// # Safety
    /// This function must guarantee that the pointer itself returns a valid VTable.
    unsafe fn metadata(&self) -> *mut dyn VariantHandle;
    
    
    /// Returns a pointer to a variant object.
    ///
    /// In other words, the implementation for this object returns a valid pointer to
    /// the variant.
    ///
    /// # Safety
    /// Both the address and the vtable pointer must be valid.
    unsafe fn raw_pointer(&self) -> *mut dyn VariantHandle;
}

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
