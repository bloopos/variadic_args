#[cfg(no_std)]
use core::{
    any::Any,
    fmt::Debug,
    mem::{ManuallyDrop, MaybeUninit},
};

use crate::{argument::VariantHandle,argument::discriminant::Discriminant, OwnedArgument};

#[cfg(not(no_std))]
use std::{
    any::Any,
    fmt::Debug,
    mem::{ManuallyDrop, MaybeUninit}
};

/// The argument kind returned from Argument::into_inner.
#[derive(Debug)]
pub enum ArgumentKind<'a>
{
    /// The inner contents are borrowed.
    Borrowed(&'a dyn Any),
    /// The inner contents are owned.
    Owned(OwnedArgument)
}

pub(super) enum RawArgument<'a>
{
    Borrowed(&'a dyn VariantHandle),
    Owned(OwnedArgument)
}

/// A union structure aimed at managing a more compact CoW instance.
pub(super) union InnerArgument<'a>
{
    /// This points to owned storage. Even if it is not initialized, it still
    /// points to valuable data, such as determining the argument's current state.
    owned: ManuallyDrop<OwnedArgument>,
    /// This pointer, if written, will only overwrite the main, owned storage. It should
    /// not overwrite the essential information, such as owned/inlined status.
    ref_: &'a dyn VariantHandle
}

impl InnerArgument<'_>
{
    /// Creates a new owned argument.
    #[inline(always)]
    pub fn new_owned(item: OwnedArgument) -> Self
    {
        let owned = ManuallyDrop::new(item);
        
        Self
        {
            owned
        }
    }
    
    
    /// Provides the discriminant for the inner storage.
    #[inline(always)]
    pub fn discriminant(&self) -> Discriminant
    {
        // Safety: We are only accessing discriminant information.
        unsafe
        {
            self.owned
                .discriminant()
        }
    }
    
    
    #[inline(always)]
    pub fn is_owned(&self) -> bool
    {
        matches!(self.discriminant(), Discriminant::Inlined | Discriminant::Allocated )
    }
    
    
    #[inline(always)]
    pub fn is_borrowed(&self) -> bool
    {
        matches!(self.discriminant(), Discriminant::Borrowed)
    }
    
    
    #[inline(always)]
    pub fn to_mut(&mut self) -> &mut dyn Any
    {
        match self.discriminant()
        {
            Discriminant::Borrowed =>
            {
                let owned : OwnedArgument = 
                unsafe
                {
                    self.ref_.clone_object()
                };
                
                *self = InnerArgument::new_owned(owned);
                
                match self.discriminant()
                {
                    Discriminant::Inlined | Discriminant::Allocated =>
                    unsafe
                    {
                        &mut *self.owned
                    },
                    _ => unreachable!()
                }
            }
            _ =>
            unsafe
            {
                &mut *self.owned
            }
        }
    }
    
    
    /// Provides a debug handle to an owned pointer.
    ///
    /// # Safety
    /// The inner contents must be owned.
    #[inline(always)]
    pub unsafe fn owned_debug_handle(&self) -> &dyn Debug
    {
        unsafe
        {
            &*self.owned
        }
    }
}


impl<'a> InnerArgument<'a>
{
    /// Creates a new instance from a borrowed trait handle.
    #[inline(always)]
    pub fn new_ref(ref_: &'a dyn VariantHandle) -> Self
    {
        // Safety: This lets us "initialize" a borrowed instance.
        let mut output : Self = unsafe { MaybeUninit::zeroed().assume_init() };
        
        output.ref_ = ref_;
        
        output
    }
    
    /// Creates a new object based around a reference to the source object.
    #[inline(always)]
    pub fn as_ref(&'a self) -> Self
    {
        let ref_ =
        match self.discriminant()
        {
            Discriminant::Borrowed => unsafe { self.ref_unchecked() },
            _ => unsafe { self.owned.raw_ref() }
        };
        
        Self::new_ref(ref_)
    }
    
    /// Takes the inner contents of the storage itself.
    ///
    /// # Safety
    /// This assumes that not only the result must be used,
    /// but the function should be called once.
    #[must_use = "Potential memory leak."]
    #[inline(always)]
    pub unsafe fn take_raw_argument(&mut self) -> RawArgument<'a>
    {
        match self.discriminant()
        {
            Discriminant::Borrowed =>
            {
                let ref_ = unsafe { self.ref_unchecked() };
                
                RawArgument::Borrowed(ref_)
            }
            _ =>
            {
                let owned =
                unsafe
                {
                    ManuallyDrop::take(&mut self.owned)
                };
                
                RawArgument::Owned(owned)
            }
        }
    }
    
    
    #[inline(always)]
    pub fn into_inner(self) -> ArgumentKind<'a>
    {
        match self.discriminant()
        {
            Discriminant::Borrowed => ArgumentKind::Borrowed(unsafe { self.ref_unchecked() }),
            _ => ArgumentKind::Owned(ManuallyDrop::into_inner(unsafe { self.owned }))
        }
    }
    
    
    /// Acquires the inner reference to the object's reference.
    ///
    /// # Safety
    /// This assumes that the storage itself is borrowed.
    #[inline(always)]
    pub unsafe fn ref_unchecked(&self) -> &'a dyn VariantHandle
    {
        unsafe
        {
            self.ref_
        }
    }
    
    
    #[inline(always)]
    pub fn to_ref(&'a self) -> &'a dyn Any
    {
        match self.discriminant()
        {
            Discriminant::Borrowed => unsafe { self.ref_unchecked() },
            _ => unsafe { self.owned.raw_ref() }
        }
    }
}
