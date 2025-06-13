mod inner;

#[cfg(no_std)]
use core::{
    any::Any,
    ops::Deref,
    mem::ManuallyDrop
};

#[cfg(not(no_std))]
use std::{
    any::Any,
    ops::Deref,
    mem::ManuallyDrop
};

use super::OwnedArgument;

use inner::{RawArgument, InnerArgument};

pub use inner::ArgumentKind;

#[repr(transparent)]
pub struct Argument<'a>
{
    inner: InnerArgument<'a>
}

impl Drop for Argument<'_>
{
    fn drop(&mut self)
    {
        let _ = unsafe { self.inner.as_argument() };
    }
}

impl<'a> Clone for Argument<'a>
{
    fn clone(&self) -> Self
    {
        let inner =
        if self.inner.is_owned()
        {
            let owned =
            {
                let raw = &raw const self.inner;
                unsafe
                {
                    (*raw.cast::<OwnedArgument>()).clone()
                }
            };
            InnerArgument::new_owned(owned)
        }
        else
        {
            InnerArgument::new_ref(unsafe { self.inner.inner_ref() })
        };
        
        Self
        {
            inner
        }
    }
}

impl Deref for Argument<'_>
{
    type Target = dyn Any;
    
    fn deref(&self) -> &dyn Any
    {
        unsafe
        {
            self.inner.to_ref()
        }
    }
}

impl Argument<'_>
{
    pub fn
    new_owned<T>(item: T) -> Self
    where
        T: Any + Clone
    {
        Self
        {
            inner: InnerArgument::new_owned(OwnedArgument::new(item))
        }
    }
    
    pub fn
    is_owned(&self) -> bool
    {
        self.inner.is_owned()
    }
    
    pub fn
    is_borrowed(&self) -> bool
    {
        self.inner.is_ref()
    }
    
    pub fn
    to_mut(&mut self) -> &mut dyn Any
    {
        unsafe
        {
            self.inner.to_mut()
        }
    }
    
    pub fn
    to_owned(&self) -> Self
    {
        if self.is_borrowed()
        {
            let ref_ = unsafe { self.inner.inner_ref() };
            let owned = ref_.clone_object();
            
            Self
            {
                inner: InnerArgument::new_owned(owned)
            }
        }
        else
        {
            self.clone()
        }
    }
    
    pub fn
    downcast_owned<T>(self) -> Result<T, Self>
    where
        T: Clone + Any
    {
        match self.inner_contents()
        {
            RawArgument::Owned(o)
            if o.is_type::<T>()
            => unsafe { Ok(o.downcast_owned_unchecked() ) },
            RawArgument::Owned(o)
            =>
            Err(Self { inner: InnerArgument::new_owned(o) }),
            RawArgument::Borrowed(b)
            =>
            Err(Self { inner: InnerArgument::new_ref(b) })
        }
    }
    
    pub unsafe fn
    downcast_owned_unchecked<T>(self) -> T
    where
        T: Clone + Any
    {
        let RawArgument::Owned(o) = self.inner_contents()
        else
        {
            panic!()
        };
        
        unsafe
        {
            o.downcast_owned_unchecked()
        }
    }
    
    pub fn
    downcast_cloned<T>(&self) -> Option<T>
    where
        T: Any + Clone
    {
        let ref_ = self.deref();
        
        if ref_.is::<T>()
        {
            let out =
            unsafe
            {
                (*(&raw const ref_).cast::<T>()).clone()
            };
            Some(out)
        } else { None }
    }
    
    pub unsafe fn
    downcast_cloned_unchecked<T>(&self) -> T
    where
        T: Any + Clone
    {
        unsafe
        {
            let ref_ = self.deref();
            (*(&raw const ref_).cast::<T>()).clone()
        }
    }
}

impl<'a> Argument<'a>
{
    pub fn new_borrowed<T>(item: &'a T) -> Self
    where
        T: Any + Clone
    {
        Self
        {
            inner: InnerArgument::new_ref(item)
        }
    }
    
    pub fn as_ref(&'a self) -> Self
    {
        Self
        {
            inner: unsafe { self.inner.as_ref() }
        }
    }
    
    fn
    inner_contents(self) -> RawArgument<'a>
    {
        let mut store = ManuallyDrop::new(self);
        
        unsafe
        {
            store.inner.as_argument()
        }
    }
    
    pub fn
    into_inner(self) -> ArgumentKind<'a>
    {
        let store = ManuallyDrop::new(self);
        
        let raw = &raw const store.inner;
        
        unsafe
        {
            raw.read().into_inner()
        }
    }
}
