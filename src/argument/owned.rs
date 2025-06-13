#[cfg(no_std)]
use alloc::{borrow, boxed::Box};

#[cfg(no_std)]
use core::{
    any::Any,
    mem,
    ops
};

#[cfg(not(no_std))]
use std::{
    any::Any,
    borrow,
    mem,
    ops
};

use super::boxed::Boxed;
use super::inlined::Inlined;
use super::variant_info::{PointerInfo, VariantHandle};

use super::boxed_argument::BoxedArgument;

pub struct OwnedArgument
{
    store: mem::MaybeUninit<Inlined>
}

impl Clone for OwnedArgument
{
    fn clone(&self) -> Self
    {
        unsafe
        {
            let ref_ =
            self.raw_pointer()
                .cast_const();
            
            (*ref_).clone_object()
        }
    }
}

impl Drop for OwnedArgument
{
    #[inline(never)]
    fn drop(&mut self)
    {
        let raw_pointer = &raw mut *self;
        
        let _ =
        unsafe
        {
            BoxedArgument::from_owned(raw_pointer)
        };
    }
}

impl PointerInfo for OwnedArgument
{
    #[inline(never)]
    unsafe fn metadata(&self) -> *mut dyn VariantHandle
    {
        if self.is_inlined()
        {
            unsafe
            {
                self.inner_inlined()
                    .metadata()
            }
        }
        else
        {
            unsafe
            {
                self.inner_boxed()
                    .metadata()
            }
        }
    }
    
    #[inline(never)]
    unsafe fn raw_pointer(&self) -> *mut dyn VariantHandle
    {
        if self.is_inlined()
        {
            unsafe
            {
                self.inner_inlined()
                    .raw_pointer()
            }
        }
        else
        {
            unsafe
            {
                self.inner_boxed()
                    .raw_pointer()
            }
        }
    }
}

impl OwnedArgument
{
    #[inline(always)]
    pub fn new<T>(item: T) -> Self
    where
        T: Any + Clone
    {
        if size_of::<T>() <= size_of::<*const ()>()
        {
            let a =
            Self
            {
                store: mem::MaybeUninit::new(Inlined::new(item))
            };
            assert!(a.is_inlined());
            a
        }
        else
        {
            let mut out =
            Self
            {
                store: mem::MaybeUninit::zeroed()
            };
            
            let boxed : Box<dyn VariantHandle> = Box::new(item);
            
            unsafe
            {
                out
                .store
                .as_mut_ptr()
                .cast::<Box<dyn VariantHandle>>()
                .write(boxed)
            };
            
            out
        }
    }
    
    /// Checks if the storage is inlined or not.
    ///
    /// This acts as a discriminant for the fake union storage OwnedVariant has.
    ///
    /// # Possible Errors
    /// Allocated Owned with Inlined discriminant.
    #[inline(always)]
    pub(crate) fn is_inlined(&self) -> bool
    {
        unsafe
        {
            self.store
                .assume_init_ref()
                .is_inlined()
        }
    }
    
    unsafe fn
    inner_inlined(&self) -> &Inlined
    {
        unsafe
        {
            self.store
                .assume_init_ref()
        }
    }
    
    unsafe fn
    inner_boxed(&self) -> &Boxed
    {
        unsafe
        {
            &*(&raw const self.store as *const _ as *const Boxed)
        }
    }
    
    /// A "wrapper" for Any::is::<T>().
    ///
    /// In case Any interferes with dereferencing the OwnedArgument, use the following function instead.
    pub fn is_type<T>(&self) -> bool
    where
        T: Any + Clone
    {
        unsafe
        {
            let metadata =
            self.metadata()
                .cast_const()
            as *const _ as *const dyn Any;
            
            (*metadata).is::<T>()
        }
    }
    
    /// 
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
    
    pub unsafe fn downcast_owned_unchecked<T>(self) -> T
    where
        T: Any + Clone
    {
        let owned = mem::ManuallyDrop::new(self);
        
        if owned.is_inlined()
        {
            let ref_ = unsafe { owned.inner_inlined() };
            let addr = &raw const ref_.store;
            unsafe
            {
                (addr as *const _ as *const T).read_unaligned()
            }
        }
        else
        {
            let pointer = unsafe { owned.inner_boxed().raw_pointer() };
            let out =
            unsafe
            {
                pointer.cast_const().cast::<T>().read_unaligned()
            };
            unsafe
            {
                super::boxed::dealloc(pointer)
            };
            out
        }
    }
    
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
    
    pub unsafe fn downcast_cloned_unchecked<T>(&self) -> T
    where
        T: Any + Clone
    {
        if self.is_inlined()
        {
            let ref_ = unsafe { self.inner_inlined() };
            let addr = &raw const ref_.store;
            unsafe
            {
                (*addr.cast::<T>()).clone()
            }
        }
        else
        {
            let pointer =
            unsafe
            {
                self.inner_boxed()
                    .raw_pointer()
            };
            
            let ref_ =
            pointer.cast_const()
                   .cast::<T>();
                   
            unsafe
            {
                (*ref_).clone()
            }
        }
    }
}

impl ops::Deref for OwnedArgument
{
    type Target = dyn Any;
    
    fn deref(&self) -> &dyn Any
    {
        unsafe
        {
            &*self.raw_pointer()
                  .cast_const()
        }
    }
}

impl ops::DerefMut for OwnedArgument
{
    fn deref_mut(&mut self) -> &mut dyn Any
    {
        unsafe
        {
            &mut *self.raw_pointer()
        }
    }
}

impl borrow::Borrow<dyn VariantHandle> for OwnedArgument
{
    fn borrow(&self) -> &dyn VariantHandle
    {
        // Method is the same, but aliasing is too complicated.
        unsafe
        {
            &*self.raw_pointer()
                  .cast_const()
        }
    }
}

impl borrow::ToOwned for dyn VariantHandle
{
    type Owned = OwnedArgument;
    
    fn to_owned(&self) -> OwnedArgument
    {
        self.clone_object()
    }
}
