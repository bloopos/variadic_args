use super::variant_info::{PointerInfo, VariantHandle};

/// A constant used for inline storage.
const INLINE_STORE : usize = size_of::<&str>() + 1;

// Inline storage for a pointer.
//
// Endianess varies for each platform, meaning that I will not write it down twice.
//
// For little endian, the storage is compressed to the following:
// ([u8; 7], VTable)
// For big endian (Possibly):
// (VTable, [u8; 7])
// 
// The last field, marker, is not used, and it's only purpose is to verify whether or
// not it is an inline storage.
#[cfg(target_endian = "big")]
#[repr(C)]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
pub(super) struct Inlined
{
    pub store: [u8; INLINE_STORE]
}

#[cfg(target_endian = "little")]
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
        // Extract the raw pointer first.
        // This is guaranteed to be Non-Null.
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
        let mut store = [1; INLINE_STORE];
        
        let item_metadata = &raw const item as *const dyn VariantHandle;
        
        let raw_pointer = store.as_mut_ptr();
        
        // Even though this has been evaluated, we cannot guarantee that it will work on all platforms.
        //
        // Particularly cases where the storage ends up being the vtable instead.
        // 
        // Let me know if a bug happens around that area.
        unsafe
        {
            // This is to "trick" the function below into copying the item's metadata via
            // converting it into raw bytes first, then offsetting it by one.
            let item_metadata =
            (&raw const item_metadata)
            .cast::<u8>();
            
            // Then, in order to prevent undefined behavior, we have to copy 15 bytes only.
            // We still keep the item's metadata, at least.
            raw_pointer.copy_from_nonoverlapping(item_metadata, size_of::<&str>());
            
            // Afterwards, we are free to write down the item itself. Albeit, in a packed manner.
            // We cannot guarantee that it will be properly aligned in the first place.
            raw_pointer
            .cast::<T>()
            .write_unaligned(item);
        }
        
        Self
        {
            store
        }
    }
    
    pub fn
    is_inlined(&self) -> bool
    {
        unsafe
        {
            (&raw const self.store).cast::<u8>().add(16).read_unaligned() > 0
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
        
        // We are using the raw values from the pointer's metadata,
        // not the actual object storage.
        let addr =
        if size_of_val(unsafe { &*metadata.cast_const() }) > 0
        {
            (&raw const self.store).addr()
        }
        else
        {
            core::ptr::NonNull::<()>::dangling().as_ptr().addr()
        };
        
        metadata.with_addr(addr)
    }
}
