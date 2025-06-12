#[cfg(no_std)]
use alloc::alloc;

#[cfg(no_std)]
use core::{
    any::Any,
    ptr::NonNull
};

#[cfg(not(no_std))]
use std::{
    alloc,
    any::Any,
    ptr::NonNull
};

use super::variant_info::{PointerInfo, VariantHandle};

/// An allocated variant.
#[repr(C)]
pub struct Boxed
{
    /// The pointer for the variant
    pointer: NonNull<dyn VariantHandle>
}

/// A wrapper for deallocating a variant.
///
/// # Safety
/// Refer to dealloc for safety concerns.
#[inline(never)]
pub unsafe fn
dealloc(pointer: *mut dyn Any)
{
    let layout = alloc::Layout::for_value(unsafe { &*pointer.cast_const() });
    
    unsafe
    {
        alloc::dealloc(pointer as *mut _ as *mut u8,
                       layout)
    };
}

impl Drop for Boxed
{
    #[inline(always)]
    fn drop(&mut self)
    {
        // Non-unique handle for dropping pointers.
        //
        // Only saftey concern is a Box inside a box.
        
        let pointer = unsafe { self.raw_pointer() };
        
        unsafe
        {
            pointer.drop_in_place();
            
            dealloc(pointer);
        }
    }
}

impl Boxed
{
    // Creates a new, allocated object.
    #[inline(always)]
    pub fn new<T>(item: T) -> Self
    where
        T: VariantHandle
    {
        let layout = alloc::Layout::new::<T>();
        
        let allocated = unsafe { alloc::alloc(layout).cast() };
        
        let Some(pointer) = NonNull::new(allocated)
        else
        {
            alloc::handle_alloc_error(layout)
        };
        
        unsafe
        {
            pointer.write(item);
        }
        
        Self
        {
            pointer
        }
    }
}

impl PointerInfo for Boxed
{
    #[inline(always)]
    unsafe fn metadata(&self) -> *mut dyn VariantHandle
    {
        self.pointer.as_ptr()
    }
    
    #[inline(always)]
    unsafe fn raw_pointer(&self) -> *mut dyn VariantHandle
    {
        self.pointer.as_ptr()
    }
}
