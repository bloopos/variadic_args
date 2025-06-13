#[cfg(no_std)]
use core::mem::MaybeUninit;

#[cfg(not(no_std))]
use std::mem::MaybeUninit;

use super::variant_info::{PointerInfo, VariantHandle};


/// A constant used for raw pointer size inside Inlined.
const INLINE_STORE : usize = size_of::<&str>();


/// Inlined storage for a pointer.
#[repr(C)]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
pub(super) struct Inlined
{
    /// The wide pointer, represented in raw bytes.
    ///
    /// It could fail in cases where Big-Endian switches up the VTable.
    store: [u8; INLINE_STORE],
    
    /// Determines whether the pointer is inlined.
    ///
    /// On regular initialization, this is always true,
    /// but uninitialized variants should always return false.
    is_inlined: bool,
    
    /// Determines whether the pointer is owned.
    /// 
    /// This always returns true if OwnedArgument gets initialized.
    is_owned: bool
}


impl Drop for Inlined
{
    #[inline(always)]
    fn drop(&mut self)
    {
        // This handles drop for a trait object.
        let pointer = unsafe { self.raw_pointer() };
        
        unsafe
        {
            pointer.drop_in_place();
        }
    }
}


impl Inlined
{
    /// Creates a new inlined stroage instance.
    #[inline]
    pub fn new<T: VariantHandle>(item: T) -> Self
    {
        let mut store = [0; INLINE_STORE];
        
        let raw_pointer = store.as_mut_ptr();
        
        let item_metadata = &raw const item as *const dyn VariantHandle;
        
        // This is so that we can copy the pointer's metadata over.
        let raw_metadata =
        (&raw const item_metadata)
        .cast::<u8>();
        
        unsafe
        {
            raw_pointer.copy_from_nonoverlapping(raw_metadata, size_of::<&str>());
            
            // After writing the metadata, we can write the actual item into the stroage itself.
            raw_pointer
            .cast::<T>()
            .write_unaligned(item);
        }
        
        Self
        {
            store,
            is_inlined: true,
            is_owned: true
        }
    }
    
    
    /// Creates an uninitialized instance for the storage.
    ///
    /// This is meant to be utilized for allocations, where the storage
    /// is actually written as a raw pointer instead of being inlined as usual.
    #[must_use = "Cannot point to a null allocation!"]
    pub fn uninit_allocated() -> MaybeUninit<Self>
    {
        let mut storage : MaybeUninit<Self> =
        MaybeUninit::zeroed();
        
        // # Safety
        // 
        // All fields inside Inlined are same to assume uninitalized, but
        // more importantly, we want to set this flag in specific. Why?
        // If we do not use it, then it would result in undefined behavior.
        unsafe
        {
            storage
            .assume_init_mut()
            .is_owned = true;
        }
        
        storage
    }
    
    
    /// Determines whether the pointer is inlined or not.
    pub fn is_inlined(&self) -> bool
    {
        self.is_inlined
    }
    
    
    /// Provides raw storage information for use in building a Discriminant.
    pub fn storage_info(&self) -> (bool, bool)
    {
        (self.is_inlined, self.is_owned)
    }
}


unsafe impl PointerInfo for Inlined
{
    #[inline(never)]
    unsafe fn metadata(&self) -> *mut dyn VariantHandle
    {
        // SAFETY: Raw pointers implement copy, meaning that
        // reading said bytes should be safe in the first place.
        // Additionally, it does not matter which platform we use,
        // we still are recieving the pointer's vtable.
        unsafe
        {
            (&raw const self.store)
            .cast::<*mut dyn VariantHandle>()
            .read_unaligned()
        }
    }
    
    
    #[inline(never)]
    unsafe fn raw_pointer(&self) -> *mut dyn VariantHandle
    {
        // Refer to metadata for safety.
        let metadata = unsafe { self.metadata() };
        
        // If the storage inside is greater than 0, we have to point it
        // to the actual address of the contents themselves.
        //
        // Otherwise, the storage is assumed to be a random pointer.
        if size_of_val(unsafe { &*metadata.cast_const() }) > 0
        {
            let addr = (&raw const self.store).addr();
            metadata.with_addr(addr)
        }
        else
        {
            metadata
        }
    }
}
