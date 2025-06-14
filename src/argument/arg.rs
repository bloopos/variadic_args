mod inner;

#[cfg(no_std)]
use core::{
    any::Any,
    fmt,
    ops::Deref,
    mem::ManuallyDrop
};

#[cfg(not(no_std))]
use std::{
    any::Any,
    fmt,
    ops::Deref,
    mem::ManuallyDrop
};

use super::{OwnedArgument, discriminant::Discriminant};

use inner::{RawArgument, InnerArgument};

pub use inner::ArgumentKind;

/// A variant item that implements Copy-on-Write.
///
/// It acts similarly to a [Cow], except for two things:
/// 1. It is encapsulated, meaning that the inner contents cannot be accessed.
/// 2. The storage is handled differently compared to [Cow], which allows for a smaller type size.
///
/// [Cow]: std::borrow::Cow
#[repr(transparent)]
pub struct Argument<'a>
{
    inner: InnerArgument<'a>
}


impl fmt::Debug for Argument<'_>
{
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self.discriminant()
        {
            Discriminant::Borrowed =>
            {
                f.debug_tuple("Argument::Borrowed")
                 .field(&self.inner.to_ref())
                 .finish()
            }
            _ =>
            {
                f.debug_tuple("Argument::Owned")
                 .field(unsafe { self.inner.owned_debug_handle() })
                 .finish()
            }
        }
    }
}


impl Drop for Argument<'_>
{
    #[inline(always)]
    fn drop(&mut self)
    {
        let _ = unsafe { self.inner.into_raw_argument() };
    }
}


impl<'a> Clone for Argument<'a>
{
    #[inline(always)]
    fn clone(&self) -> Self
    {
        let inner =
        match self.discriminant()
        {
            Discriminant::Borrowed =>
            {
                let ref_ =
                unsafe {
                    self.inner
                        .ref_unchecked()
                };
                
                InnerArgument::new_ref(ref_)
            }
            _ =>
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
    
    #[inline(always)]
    fn deref(&self) -> &dyn Any
    {
        self.inner
            .to_ref()
    }
}

impl Argument<'_>
{
    /// Creates a new owned Argument.
    #[inline(always)]
    pub fn new_owned<T>(item: T) -> Self
    where
        T: Any + Clone
    {
        let owned = OwnedArgument::new(item);
        
        Self
        {
            inner: InnerArgument::new_owned(owned)
        }
    }
    
    /// Checks if the argument is owned.
    #[inline(always)]
    pub fn is_owned(&self) -> bool
    {
        self.inner
            .is_owned()
    }
    
    /// Checks if the argument is borrowed.
    #[inline(always)]
    pub fn is_borrowed(&self) -> bool
    {
        self.inner
            .is_borrowed()
    }
    
    /// Returns a mutable reference to the item itself.
    ///
    /// If the inner contents are borrowed, this creates a new
    /// owned instance first before returning the reference itself.
    #[inline(always)]
    pub fn to_mut(&mut self) -> &mut dyn Any
    {
        self.inner
            .to_mut()
    }
    
    #[inline(always)]
    fn discriminant(&self) -> Discriminant
    {
        self.inner
            .discriminant()
    }
    
    /// Clones the inner contents of the object, returning an owned argument.
    #[inline(always)]
    pub fn to_owned(&self) -> Self
    {
        match self.discriminant()
        {
            Discriminant::Borrowed =>
            {
                let ref_ =
                unsafe
                {
                    self.inner
                        .ref_unchecked()
                };
                
                let owned = ref_.clone_object();
                
                Self
                {
                    inner: InnerArgument::new_owned(owned)
                }
            }
            _ => self.clone()
        }
    }
    
    /// Downcasts an owned argument into type T, returning a result.
    ///
    /// # Return values
    /// Ok(T): The argument gets consumed and returns the inner contents.
    /// Err(Self): Either the argument is not of type T or the argument itself is not owned.
    #[inline(always)]
    pub fn downcast_owned<T>(self) -> Result<T, Self>
    where
        T: Clone + Any
    {
        match self.inner_contents()
        {
            RawArgument::Owned(owned)
            if owned.is_type::<T>() =>
            unsafe
            {
                Ok(owned.downcast_owned_unchecked())
            }
            
            RawArgument::Owned(o)
            =>
            Err(Self { inner: InnerArgument::new_owned(o) }),
            
            RawArgument::Borrowed(b)
            =>
            Err(Self { inner: InnerArgument::new_ref(b) })
        }
    }
    
    /// Downcasts an owned argument into type T, without any checks.
    ///
    /// # Safety
    /// The argument must both be owned and of type T.
    ///
    /// # Panics
    /// The function will panic if the argument itself is not owned.
    #[inline(always)]
    pub unsafe fn downcast_owned_unchecked<T>(self) -> T
    where
        T: Clone + Any
    {
        debug_assert!(self.is_owned());
        
        let RawArgument::Owned(contents) : OwnedArgument =
        self.inner_contents()
        else
        {
            #[cfg(debug_assertions)]
            {
                unreachable!()
            }
            #[cfg(not(debug_assertions))]
            {
                panic!()
            }
        };
        
        debug_assert!(contents.is_type::<T>());
        
        unsafe
        {
            contents.downcast_owned_unchecked()
        }
    }
    
    /// Downcasts the argument into a cloned object of type T.
    ///
    /// Returns None if the object's type is not T.
    #[inline(always)]
    pub fn downcast_cloned<T>(&self) -> Option<T>
    where
        T: Any + Clone
    {
        #[allow(clippy::manual_map)]
        match self.downcast_ref::<T>()
        {
            Some(t) => Some(t.clone()),
            None => None
        }
    }
    
    /// Binding to downcast a reference to T without checks.
    ///
    /// This is similar to Any::downcast_ref_unchecked, except for
    /// the fact that we can use it outside of nightly. When the former
    /// gets stabilized, this function will get replaced.
    ///
    /// # Safety
    /// Assumes that the contents are of type T.
    #[inline(always)]
    unsafe fn downcast_ref_unchecked<T>(&self) -> &T
    where
        T: Any + Clone
    {
        let binding = self.inner.to_ref();
        
        debug_assert!(binding.is::<T>());
        
        unsafe
        {
            &*(binding as *const dyn Any as *const T)
        }
    }
    
    /// Downcasts the argument into a cloned object of T without checking it first.
    ///
    /// # Safety
    /// Assumes that the contents are of type T.
    #[inline(always)]
    pub unsafe fn downcast_cloned_unchecked<T>(&self) -> T
    where
        T: Any + Clone
    {
        unsafe
        {
            self.downcast_ref_unchecked::<T>().clone()
        }
    }
}

impl<'a> Argument<'a>
{
    /// Creates a borrowed argument of item T.
    #[inline(always)]
    pub fn new_borrowed<T>(item: &'a T) -> Self
    where
        T: Any + Clone
    {
        Self
        {
            inner: InnerArgument::new_ref(item)
        }
    }
    
    /// Creates a borrowed reference to the source argument.
    #[inline(always)]
    pub fn as_ref(&'a self) -> Self
    {
        Self
        {
            inner: self.inner.as_ref()
        }
    }
    
    /// Consumes the argument, returning a wrapper to the inner argument itself.
    #[inline(always)]
    fn inner_contents(self) -> RawArgument<'a>
    {
        let mut store = ManuallyDrop::new(self);
        
        unsafe
        {
            store.inner.into_raw_argument()
        }
    }
    
    /// Consumes the argument itself, returning what kind of argument it is.
    #[inline(always)]
    pub fn into_inner(self) -> ArgumentKind<'a>
    {
        let store = ManuallyDrop::new(self);
        
        let raw = &raw const store.inner;
        
        unsafe
        {
            raw.read().into_inner()
        }
    }
}

impl From<OwnedArgument> for Argument<'_>
{
    fn from(item: OwnedArgument) -> Self
    {
        Self
        {
            inner: InnerArgument::new_owned(item)
        }
    }
}
