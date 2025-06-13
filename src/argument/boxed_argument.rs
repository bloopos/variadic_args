#[cfg(no_std)]
use alloc::boxed::Box;

use super::{
    discriminant::Discriminant,
    inlined::Inlined,
    OwnedArgument,
    VariantHandle
};

/// A raw alias for the contents inside an owned argument.
pub enum BoxedArgument
{
    /// The contents have been allocated.
    Allocated(Box<dyn VariantHandle>),
    /// The contents have been inlined.
    Inlined(Inlined)
}

impl BoxedArgument
{
    /// Acquires the inner pointer from an owned argument.
    ///
    /// # SAFETY
    /// The following must be guaranteed for defined behavior.
    ///  * The pointer must be owned.
    ///  * The pointer must point to an actual OwnedArgument.
    pub unsafe fn from_owned(owned: *mut OwnedArgument) -> Self
    {
        debug_assert!(!owned.is_null());
        
        let discriminant =
        {
            let ref_ =
            unsafe { &*owned.cast_const() };
            
            ref_.owned_discriminant()
        };
        
        // Safety:
        //
        // As long as the discriminant matches correctly,
        // the following are assumed to be said state.
        match discriminant
        {
            Discriminant::Inlined =>
            {
                let pointer : *mut Inlined =
                owned.cast();
                
                let inlined = unsafe { pointer.read() };
                
                Self::Inlined(inlined)
            }
            Discriminant::Allocated =>
            {
                let object_pointer : *mut dyn VariantHandle =
                unsafe
                {
                    owned
                    .cast::<*mut dyn VariantHandle>()
                    .read()
                };
                
                let allocated = unsafe { Box::from_raw(object_pointer) };
                
                Self::Allocated(allocated)
            }
            _ => unreachable!()
        }
    }
}
