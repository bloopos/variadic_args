#[cfg(no_std)]
use ::alloc::{alloc, boxed::Box};

#[cfg(no_std)]
use core::{
    any::Any,
    fmt,
    mem,
    ptr::NonNull,
    ops
};

#[cfg(not(no_std))]
use std::{
    alloc,
    fmt,
    any::Any,
    mem,
    ptr::NonNull,
    ops
};

use super::{
    discriminant::Discriminant,
    boxed_argument::BoxedArgument,
    inlined::Inlined,
    variant_info::VariantHandle
};

/// An owned argument.
///
/// This carries a generic item that implements both Any and Clone.
/// In addition, depending on the storage itself, it is able to implement
/// items whose size is no more than 8 bytes for 64-bit systems (or 4 for 32-bit systems).
pub struct OwnedArgument
{
    /// Pointer storage. This acts as a wrapper for both
    /// inlined and boxed storage. Due to inline behavior, we
    /// cannot use this pointer directly.
    pointer: *mut dyn VariantHandle,
    inlined: bool,
    owned: bool
}

impl fmt::Debug for OwnedArgument
{
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut current = f.debug_struct("OwnedArgument");
                 
        current.field("is_inlined", &self.inlined);
        
        let ref_ : &dyn Any =
        unsafe { self.pointer().as_ref() };
        
        current.field("storage", &ref_);
        
        current.finish()
    }
}

impl Clone for OwnedArgument
{
    #[inline(always)]
    fn clone(&self) -> Self
    {
        unsafe
        {
            self.pointer()
                .as_ref()
                .clone_object()
        }
    }
}

impl Drop for OwnedArgument
{
    #[inline(always)]
    fn drop(&mut self)
    {
        let _ =
        unsafe
        {
            BoxedArgument::from_owned(self.pointer,
                                      self.owned_discriminant())
        };
    }
}


#[cfg(debug_assertions)]
fn pointer_matches<T>(pointer: *mut dyn VariantHandle) -> bool
where
    T: Any + Clone
{
    assert!(!pointer.is_null());
    
    let pointer : *const dyn Any =
    pointer.cast_const() as *const _ as *const dyn Any;
    
    let ref_ = unsafe { &*pointer };
    
    ref_.is::<T>()
}

impl OwnedArgument
{
    /// Creates a new OwnedArgument based around a generic item.
    ///
    /// If the size of said item is less than 8 bytes for 64-bit systems (4 for 32-bit systems),
    /// then the storage is inlined. Otherwise, the storage gets allocated instead.
    #[inline(always)]
    pub fn new<T>(item: T) -> Self
    where
        T: Any + Clone
    {
        if size_of::<T>() <= size_of::<*const ()>()
        {
            let mut store = mem::MaybeUninit::<*mut dyn VariantHandle>::new((&raw const item).cast_mut());

            unsafe
            {
                store.as_mut_ptr().cast::<T>().write(item);
            }

            Self
            {
                pointer:
                unsafe
                {
                    store.assume_init()
                },
                inlined: true,
                owned: true
            }
        }
        else
        {
            let boxed : Box<dyn VariantHandle> = Box::new(item);
            
            let pointer = Box::into_raw(boxed);
            
            Self
            {
                pointer,
                inlined: false,
                owned: true
            }
        }
    }

    #[inline(always)]
    unsafe fn pointer_metadata(&self) -> *mut dyn VariantHandle
    {
        self.pointer
    }

    #[inline(always)]
    fn pointer(&self) -> NonNull<dyn VariantHandle>
    {
        match self.owned_discriminant()
        {
            Discriminant::Inlined =>
            unsafe { self.inner_inlined().pointer() },
            Discriminant::Allocated =>
            unsafe { NonNull::new_unchecked(self.pointer) },
            _ => unreachable!()
        }
    }
    
    /// Acquires the discriminant of the OwnedPointer.
    ///
    /// This should not return Discriminant::Borrowed.
    #[inline(always)]
    pub(crate) fn owned_discriminant(&self) -> Discriminant
    {
        Discriminant::from_owned(self.inlined)
    }
    
    /// Acquires the discriminant based around the OwnedPointer's storage information.
    #[inline(always)]
    pub(crate) fn discriminant(&self) -> Discriminant
    {
        Discriminant::from_info((self.inlined, self.owned))
    }
    
    /// Checks if the storage is inlined or not.
    ///
    /// This is only used for testing purposes.
    #[cfg(test)]
    pub(crate) fn is_inlined(&self) -> bool
    {
        self.inlined
    }
    
    /// Acquires the inner pointer to the inlined storage.
    ///
    /// # Safety
    /// For accessing information, such as owned and inlined status,
    /// this is guaranteed to be safe. Otherwise, this function assumes
    /// that the storage is inlined.
    #[inline(always)]
    unsafe fn inner_inlined(&self) -> &Inlined
    {
        unsafe
        {
            &*(&raw const self.pointer)
                .cast::<Inlined>()
        }
    }

    /// A "wrapper" for `Any::is::<T>()`.
    ///
    /// In case Any interferes with dereferencing the OwnedArgument, use the following function instead.
    #[inline(always)]
    pub fn is_type<T>(&self) -> bool
    where
        T: Any + Clone
    {
        unsafe
        {
            let metadata : *const dyn Any =
            self.pointer_metadata().cast_const() as *const _ as *const dyn Any;

            (*metadata).is::<T>()
        }
    }
    
    /// Acquires a raw reference handle to the object itself.
    ///
    /// This is useful for internally creating references to VariantHandle.
    #[inline(always)]
    pub(crate) fn raw_ref<'a>(&'a self) -> &'a dyn VariantHandle
    {
        unsafe
        {
            self.pointer()
                .as_ref()
        }
    }
    
    /// Downcasts the object into an owned instance.
    ///
    /// # Return values:
    /// Ok(val): The value matches is T, and the previous storage frees itself.
    /// Err(self): The value does not match T, the inner value should remain identical.
    #[inline(always)]
    pub fn downcast_owned<T>(self) -> Result<T, Self>
    where
        T: Any + Clone
    {
        if self.is_type::<T>()
        {
            unsafe
            {
               Ok(self.downcast_owned_unchecked())
            }
        } else { Err(self) }
    }
    
    /// Downcasts the inner value into T without checking it first.
    ///
    /// # Safety
    /// This assumes that the type supplied is, in fact, T.
    #[inline(always)]
    pub unsafe fn downcast_owned_unchecked<T>(self) -> T
    where
        T: Any + Clone
    {
        let owned = mem::ManuallyDrop::new(self);
        
        let boxed =
        unsafe
        {
            BoxedArgument::from_owned(owned.pointer,
                                      owned.owned_discriminant())
        };
        
        match boxed
        {
            BoxedArgument::Allocated(a) =>
            {
                let raw_pointer = Box::into_raw(a);
                
                #[cfg(debug_assertions)]
                {
                    assert!(pointer_matches::<T>(raw_pointer));
                }
                
                let output =
                {
                    let pointer : *mut T =
                    raw_pointer.cast();
                    
                    unsafe
                    {
                        pointer.read_unaligned()
                    }
                };
                
                let layout = alloc::Layout::new::<T>();
                
                unsafe
                {
                    alloc::dealloc(raw_pointer.cast::<u8>(),
                                   layout);
                }
                
                output
            }
            BoxedArgument::Inlined(i) =>
            {
                #[cfg(debug_assertions)]
                {
                    let raw_pointer = i.pointer().as_ptr();
                    assert!(pointer_matches::<T>(raw_pointer));
                }
                
                let store = mem::ManuallyDrop::new(i);
                
                let pointer = &raw const store;
                
                unsafe
                {
                    pointer.cast::<T>().read()
                }
            }
        }
    }
    
    /// Downcasts a reference of the OwnedArgument before returning the cloned contents of the inner value:
    ///
    /// # Return values
    /// Some(v): The cloned object is of type T,
    /// None: OwnedArgument is not type T
    #[inline(always)]
    pub fn downcast_cloned<T>(&self) -> Option<T>
    where
        T: Any + Clone
    {
        if self.is_type::<T>()
        {
            unsafe
            {
                Some(self.downcast_cloned_unchecked())
            }
        }
        else { None }
    }
    
    /// Returns the cloned contents of the inner type of an OwnedArgument without performing any checks.
    ///
    /// # Safety
    /// This assumes that the OwnedArgument is type T.
    #[inline(always)]
    pub unsafe fn downcast_cloned_unchecked<T>(&self) -> T
    where
        T: Any + Clone
    {
        let pointer = self.pointer();
        
        #[cfg(debug_assertions)]
        {
            assert!(pointer_matches::<T>(pointer.as_ptr()));
        }
        
        unsafe
        {
            pointer.cast::<T>().as_ref().clone()
        }
    }
}


impl ops::Deref for OwnedArgument
{
    type Target = dyn Any;
    
    #[inline(always)]
    fn deref(&self) -> &dyn Any
    {
        unsafe
        {
            self.pointer()
                .as_ref()
        }
    }
}


impl ops::DerefMut for OwnedArgument
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut dyn Any
    {
        unsafe
        {
            self.pointer()
                .as_mut()
        }
    }
}

