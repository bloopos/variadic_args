use super::variant_info::{PointerInfo, VariantHandle};

#[cfg(no_std)]
use core::mem::MaybeUninit;

#[cfg(not(no_std))]
use std::mem::MaybeUninit;

/// A constant used for inline storage.
const INLINE_STORE : usize = size_of::<&str>();

// Inline storage for a pointer.
//
// Endianess varies for each platform, so it's likely that
// this all falls apart against Big Endian.
#[repr(C)]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
pub(super) struct Inlined
{
    store: [u8; INLINE_STORE],
    is_inlined: bool,
    is_owned: bool
}

impl Drop for Inlined
{
    #[inline(always)]
    fn drop(&mut self)
    {
        let pointer = unsafe { self.raw_pointer() };
        
        unsafe
        {
            pointer.drop_in_place();
        }
    }
}

impl Inlined
{
    /// Creates a new inlined pointer.
    ///
    /// It creates a new storage before storing both the item itself, as well as its
    /// metadata, into the storage itself.
    #[inline]
    pub fn new<T: VariantHandle>(item: T) -> Self
    {
        // This guarantees that the storage we write into is inlined.
        let mut store = [0; INLINE_STORE];
        
        let raw_pointer = store.as_mut_ptr();
        
        let item_metadata = &raw const item as *const dyn VariantHandle;
        
        let raw_metadata =
        (&raw const item_metadata)
        .cast::<u8>();
        
        unsafe
        {
            raw_pointer.copy_from_nonoverlapping(raw_metadata, size_of::<&str>());
            
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
    
    pub fn
    uninit_allocated() -> MaybeUninit<Self>
    {
        let mut storage : MaybeUninit<Self> =
        MaybeUninit::zeroed();
        
        unsafe
        {
            storage.assume_init_mut().is_owned = true;
        }
        
        storage
    }
    pub fn
    is_inlined(&self) -> bool
    {
        self.is_inlined
    }
    
    pub fn
    storage_info(&self) -> (bool, bool)
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
