#[cfg(no_std)]
use core::ptr::NonNull;

use super::VariantHandle;

#[cfg(not(no_std))]
use std::ptr::NonNull;


#[repr(transparent)]
pub struct Inlined
{
    contents: *mut dyn VariantHandle
}


impl Drop for Inlined
{
    fn drop(&mut self)
    {
        let pointer = self.pointer();

        // Safety: We are properly freeing the pointer.
        unsafe
        {
            pointer.drop_in_place();
        }
    }
}

impl Inlined
{
    /// Provides the metadata for the inline storage.
    ///
    /// # Safety
    /// This is not a valid pointer to the actual storage.
    /// All it provides is the vtable itself, which reading
    /// layout information should be safe.
    pub unsafe fn metadata(&self) -> *mut dyn VariantHandle
    {
        self.contents
    }


    pub fn storage(&self) -> *const u8
    {
        (&raw const self.contents).cast()
    }


    pub fn pointer(&self) -> NonNull<dyn VariantHandle>
    {
        let metadata =
        unsafe
        {
            self.metadata()
        };

        let pointer =
        if size_of_val(unsafe { &*metadata }) > 0
        {
            let address =
            self.storage()
            .addr();

            metadata.with_addr(address)
        }
        else
        {
            metadata
        };

        debug_assert!(!pointer.is_null());

        unsafe
        {
            NonNull::new_unchecked(pointer)
        }
    }
}


impl From<*mut dyn VariantHandle> for Inlined
{
    fn from(pointer: *mut dyn VariantHandle) -> Self
    {
        Self
        {
            contents: pointer
        }
    }
}
