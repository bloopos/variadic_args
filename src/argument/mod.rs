//finished
mod variant_info;

mod boxed;
//finished
mod inlined;
mod owned;

// Unsure what to do about it.
//mod borrowed_arg;

#[cfg(feature = "no_std")]
use alloc::borrow::Cow;
#[cfg(feature = "no_std")]
use core::{
    any::Any,
    ops::Deref
};

#[cfg(not(feature = "no_std"))]
use std::{
    any::Any,
    borrow::Cow,
    ops::Deref
};

pub use owned::OwnedArgument;

use variant_info::{PointerInfo};

pub(crate) use variant_info::VariantHandle;

/// A variadic object wrapped around an encapsulated Cow.
///
/// This aims to save space via a similar method with SmartString, and that
/// is to inline the owned storage instead. That way, we can easily save
/// more storage 
#[derive(Clone)]
#[repr(transparent)]
pub struct Argument<'a>(Cow<'a, dyn VariantHandle>);

impl Deref for Argument<'_>
{
    type Target = dyn Any;
    
    fn deref(&self) -> &dyn Any
    {
        self.0.deref()
    }
}

impl<'a> Argument<'a>
{
    pub fn new_ref<T>(item: &'a T) -> Self
    where
        T: Any + Clone
    {
        Self(Cow::Borrowed(item))
    }
    
    pub fn as_ref(&'a self) -> Self
    {
        let ref_ =
        match self.0
        {
            Cow::Borrowed(b) => b,
            Cow::Owned(ref o) => unsafe { &*o.raw_pointer().cast_const() }
        };
        
        Self(Cow::Borrowed(ref_))
    }
}

impl Argument<'_>
{
    pub fn new_owned<T>(item: T) -> Self
    where
        T: Any + Clone
    {
        Self(Cow::Owned(OwnedArgument::new(item)))
    }
    
    pub fn is_owned(&self) -> bool
    {
        match self.0
        {
            Cow::Owned(_) => true,
            Cow::Borrowed(_) => false
        }
    }
    
    pub fn is_borrowed(&self) -> bool
    {
        match self.0
        {
            Cow::Borrowed(_) => true,
            Cow::Owned(_) => false
        }
    }
    
    pub fn to_mut(&mut self) -> &mut dyn Any
    {
        self.0.to_mut()
    }
    
    pub fn downcast_owned<T>(self) -> Result<T, Self>
    where
        T: Any + Clone
    {
        match self.0
        {
            // Owned storage is not only a mandatory requirement,
            // but we also have to make sure that we are downcasting the
            // correct type.
            Cow::Owned(o)
            if o.is_type::<T>()
            =>
            // Safety: We have already guaranteed the following,
            // so downcast it without checking it.
            unsafe
            {
                Ok(o.downcast_owned_unchecked())
            },
            e => Err(Self(e))
        }
    }
    
    pub unsafe fn downcast_owned_unchecked<T>(self) -> T
    where
        T: Any + Clone
    {
        match self.0
        {
            Cow::Owned(o)
            => unsafe { o.downcast_owned_unchecked() },
            // When running unsafe code, technically, it is possible to
            // reach. But at the same time, I am not sure what kind of error
            // message to implement.
            _ => unimplemented!()
        }
    }
    
    pub fn downcast_cloned<T>(&self) -> Option<T>
    where
        T: Any + Clone
    {
        
        match self.0
        {
            Cow::Owned(ref o)
            if o.is_type::<T>()
            =>
            unsafe { Some(o.downcast_cloned_unchecked()) }
            Cow::Borrowed(b)
            if (b as &dyn Any).is::<T>()
            =>
            unsafe
            {
                let addr = &raw const b as *const T;
                Some((&*addr).clone())
            }
            _ => None
        }
    }
    
    pub unsafe fn downcast_cloned_unchecked<T>(&self) -> T
    where
        T: Any + Clone
    {
        // It is the same as above, except for the fact that
        // we do not perform any sort of checks.
        //
        // Why not use it in the previous method? That is to
        // run the switch case in one go. So any safety checks
        // assume that the variadic item IS type T.
        match self.0
        {
            Cow::Owned(ref o) =>
            unsafe { o.downcast_cloned_unchecked() },
            // 
            Cow::Borrowed(b)
            =>
            unsafe
            {
                let addr = &raw const b as *const T;
                (&*addr).clone()
            }
        }
    }
}

