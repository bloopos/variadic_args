#[cfg(no_std)]
use core::{
    any::Any,
    mem::ManuallyDrop
};

use crate::{VariantHandle, OwnedArgument};

#[cfg(not(no_std))]
use std::{
    any::Any,
    mem::ManuallyDrop
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
    owned: ManuallyDrop<OwnedArgument>,
    flags: (*mut dyn VariantHandle, bool, bool),
    ref_: &'a dyn VariantHandle
}

impl InnerArgument<'_>
{
    pub fn new_owned(item: OwnedArgument) -> Self
    {
        let owned = ManuallyDrop::new(item);
        
        let mut output = Self { owned };
        
        #[cfg(debug_assertions)]
        {
            unsafe
            {
                let pointer = &raw const output;
                let value =
                pointer
                .cast::<u8>()
                .add(size_of::<&str>())
                .read();
                assert!(value < 2);
            }
        }
        
        output.flags.2 = true;
        
        output
    }
    
    pub fn is_owned(&self) -> bool
    {
        unsafe
        {
            self.flags.2
        }
    }
    
    pub fn is_ref(&self) -> bool
    {
        !self.is_owned()
    }
    
    pub unsafe fn to_ref(&self) -> &dyn Any
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
                self.ref_
            }
        }
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
        let mut output = Self { ref_ };
        
        output.flags.2 = false;
        
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
}
