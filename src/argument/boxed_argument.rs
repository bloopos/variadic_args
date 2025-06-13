#[cfg(no_std)]
use alloc::boxed::Box;

use super::{
    VariantHandle, OwnedArgument,
    inlined::Inlined
};

pub enum BoxedArgument
{
    Allocated(Box<dyn VariantHandle>),
    Inlined(Inlined)
}

impl BoxedArgument
{
    /// Acquires the inner pointer from an owned argument.
    pub unsafe fn from_owned(owned: *mut OwnedArgument) -> Self
    {
        debug_assert!(!owned.is_null());
        
        let is_inlined = unsafe { (*owned.cast_const()).is_inlined() };
        
        if is_inlined
        {
            let pointer : *mut Inlined = owned.cast();
            
            let inlined = unsafe { pointer.read() };
            
            Self::Inlined(inlined)
        }
        else
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
    }
}
