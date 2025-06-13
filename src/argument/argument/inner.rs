#[cfg(no_std)]
use core::{
    any::Any,
    mem::{ManuallyDrop, MaybeUninit},
};

use crate::{argument::VariantHandle,argument::discriminant::Discriminant, OwnedArgument};

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
        
        let output = Self { owned };
        
        output
    }
    
    pub fn discriminant(&self) -> Discriminant
    {
        unsafe
        {
            self.owned.discriminant()
        }
    }
    
    pub fn is_owned(&self) -> bool
    {
        matches!(self.discriminant(), Discriminant::Inlined | Discriminant::Allocated )
    }
    
    pub fn is_ref(&self) -> bool
    {
        matches!(self.discriminant(), Discriminant::Borrowed)
    }
    
    pub fn to_mut(&mut self) -> &mut dyn Any
    {
        match self.discriminant()
        {
            Discriminant::Borrowed =>
            {
                let owned = unsafe { self.ref_.clone_object() };
                *self = Self::new_owned(owned);
                
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
}

impl<'a> InnerArgument<'a>
{
    pub fn new_ref(ref_: &'a dyn VariantHandle) -> Self
    {
        let mut output : Self = unsafe { MaybeUninit::zeroed().assume_init() };
        
        output.ref_ = ref_;
        
        output
    }
    
    pub fn as_ref(&'a self) -> Self
    {
        let ref_ =
        match self.discriminant()
        {
            Discriminant::Borrowed => unsafe { self.inner_ref() },
            _ => unsafe { self.owned.raw_ref() }
        };
        
        Self::new_ref(ref_)
    }
    
    pub unsafe fn as_argument(&mut self) -> RawArgument<'a>
    {
        match self.discriminant()
        {
            Discriminant::Borrowed =>
            {
                let ref_ = unsafe { self.inner_ref() };
                
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
    
    pub fn to_ref(&'a self) -> &'a dyn Any
    {
        match self.discriminant()
        {
            Discriminant::Borrowed => unsafe { self.inner_ref() },
            _ => unsafe { self.owned.raw_ref() }
        }
    }
}
