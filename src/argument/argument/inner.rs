#[cfg(no_std)]
use core::{
    any::Any,
    mem::{ManuallyDrop, MaybeUninit},
};

use crate::{argument::VariantHandle, OwnedArgument};

#[cfg(not(no_std))]
use std::{
    any::Any,
    mem::{ManuallyDrop, MaybeUninit}
};

pub enum ArgumentKind<'a>
{
    Borrowed(&'a dyn Any),
    Owned(OwnedArgument)
}

pub(super) enum RawArgument<'a>
{
    Borrowed(&'a dyn VariantHandle),
    Owned(OwnedArgument)
}

pub(super) union InnerArgument<'a>
{
    // This points to owned storage.
    owned: ManuallyDrop<OwnedArgument>,
    // 
    ref_: &'a dyn VariantHandle
}

impl InnerArgument<'_>
{
    pub fn new_owned(item: OwnedArgument) -> Self
    {
        let owned = ManuallyDrop::new(item);
        
        let mut output = Self { owned };
        
        output
    }
    
    pub fn is_owned(&self) -> bool
    {
        unsafe
        {
            self.owned.is_owned()
        }
    }
    
    pub fn is_ref(&self) -> bool
    {
        !self.is_owned()
    }
    
    pub unsafe fn to_mut(&mut self) -> &mut dyn Any
    {
        if self.is_ref()
        {
            let owned = unsafe { self.ref_.clone_object() };
            *self = Self::new_owned(owned);
        }
        
        debug_assert!(self.is_owned());
        
        unsafe
        {
            &mut *self.owned
        }
    }
    
}

impl<'a> InnerArgument<'a>
{
    pub fn new_ref(ref_: &'a dyn VariantHandle) -> Self
    {
        let mut output : Self = unsafe { MaybeUninit::zeroed().assume_init() };
        
        output.ref_ = ref_;
        
        output
    }
    
    pub unsafe fn as_ref(&'a self) -> Self
    {
        let ref_ =
        if self.is_ref()
        {
            unsafe { self.inner_ref() }
        }
        else { unsafe { self.owned.raw_ref() } };
        
        Self::new_ref(ref_)
    }
    
    pub unsafe fn as_argument(&mut self) -> RawArgument<'a>
    {
        if self.is_owned()
        {
            let owned =
            unsafe
            {
                ManuallyDrop::take(&mut self.owned)
            };
            
            RawArgument::Owned(owned)
        }
        else
        {
            let ref_ =
            unsafe
            {
                self.ref_
            };
            RawArgument::Borrowed(ref_)
        }
    }
    
    pub unsafe fn into_inner(self) -> ArgumentKind<'a>
    {
        if self.is_owned()
        {
            ArgumentKind::Owned(ManuallyDrop::into_inner(unsafe { self.owned }))
        }
        else
        {
            ArgumentKind::Borrowed(unsafe { self.ref_ })
        }
    }
    
    pub unsafe fn inner_ref(&self) -> &'a dyn VariantHandle
    {
        unsafe
        {
            self.ref_
        }
    }
    
    pub unsafe fn to_ref(&'a self) -> &'a dyn Any
    {
        if self.is_owned()
        {
            unsafe
            {
                self.owned.raw_ref()
            }
        }
        else
        {
            unsafe
            {
                self.inner_ref()
            }
        }
    }
}
