use super::variant_info::{PointerInfo, VariantHandle};

/// A constant used for inline storage.
const INLINE_STORE : usize = size_of::<&str>() + 1;

// Inline storage for a pointer.
//
// Endianess varies for each platform, so it's likely that
// this all falls apart against Big Endian.
#[repr(C)]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
pub(super) struct Inlined
{
    pub store: [u8; INLINE_STORE]
}

impl Drop for Inlined
{
    #[inline(always)]
    fn drop(&mut self)
    {
        let pointer = unsafe { self.raw_pointer() };
        
        unsafe
        {
            pointer.drop_in_place()
        };
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
        let mut store = [1; INLINE_STORE];
        
        let raw_pointer = store.as_mut_ptr();
        
        if size_of::<T>() > 0
        {
            let item_metadata = &raw const item as *const dyn VariantHandle;
            
            unsafe
            {
                let item_metadata =
                (&raw const item_metadata)
                .cast::<u8>();
                
                raw_pointer.copy_from_nonoverlapping(item_metadata, size_of::<&str>());
                
                raw_pointer
                .cast::<T>()
                .write_unaligned(item);
            }
        }
        else
        {
            let metadata =
            core::ptr::dangling_mut::<T>() as *mut _ as *mut dyn VariantHandle;
            
            unsafe
            {
                raw_pointer.copy_from_nonoverlapping(metadata.cast(), size_of::<&str>());
            }
        }
        
        
        Self
        {
            store
        }
    }
    
    pub fn
    is_inlined(&self) -> bool
    {
        // Safety:
        // Should be initialized with ones. Anything greater than that
        // would be a concern.
        unsafe
        {
            *(&raw const self.store).cast::<bool>().add(16)
        }
    }
}

impl PointerInfo for Inlined
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
