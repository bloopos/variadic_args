#[cfg(no_std)]
use alloc::{
    boxed::Box,
    vec::IntoIter as IntoVecIter
};

#[cfg(no_std)]
use core::{
    mem::ManuallyDrop,
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
    slice::{Iter, IterMut}
};

#[cfg(not(no_std))]
use std::{
    mem::ManuallyDrop,
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
    slice::{Iter, IterMut},
    vec::IntoIter as IntoVecIter
};

use crate::Argument;

const MAX_ARG_COUNT : usize = 1024;

pub struct Arguments<'a>
{
    table: *mut Argument<'a>,
    count: u16
}

impl Drop for Arguments<'_>
{
    fn drop(&mut self)
    {
        let _ = unsafe { box_from_arguments(self.table, self.count) };
    }
}

impl Arguments<'_>
{
    fn into_owned(&mut self)
    {
        let ref_ =
        unsafe
        {
            let pointer = raw_slice(self.table, self.count);
            &mut *pointer
        };
        
        ref_.iter_mut().for_each(|arg| { let _ = arg.to_mut(); });
    }
}

impl<'a> Arguments<'a>
{
    pub fn
    from_slice(contents: &[Argument<'a>])
    -> Option<Self>
    {
        let count = contents.len();
        
        if count > MAX_ARG_COUNT
        {
            return None;
        }
        
        let boxed = Box::<[Argument<'a>]>::from(contents);
        
        Some(Self
        {
            table: Box::into_raw(boxed).cast(),
            count: count as u16
        })
    }
    
    pub fn
    from_boxed(contents: Box<[Argument<'a>]>)
    -> Result<Self, Box<[Argument<'a>]>>
    {
        let count = contents.len();
        
        if count > MAX_ARG_COUNT
        {
            return Err(contents);
        }
        
        let raw_pointer = Box::into_raw(contents);
        
        
        Ok(
            Self
            {
                table: raw_pointer.cast(),
                count: count as u16
            }
        )
    }
    
    pub fn
    iter(&self) -> Iter<'a, Argument<'a>>
    {
        let slice = slice_from_raw_parts(self.table.cast(),
                                         self.count as usize);
        
        unsafe
        {
            (*slice).iter()
        }
    }
    
    pub fn
    iter_mut(&mut self) -> IterMut<'a, Argument<'a>>
    {
        self.into_owned();
        
        let slice = raw_slice(self.table, self.count);
        
        unsafe
        {
            (&mut *slice).iter_mut()
        }
    }
}

fn
raw_slice<'a>(table: *mut Argument<'a>,
              count: u16)
-> *mut [Argument<'a>]
{
    slice_from_raw_parts_mut(table, count as usize)
}

unsafe fn
box_from_arguments<'a>(table: *mut Argument<'a>,
                   count: u16)
-> Box<[Argument<'a>]>
{
    let raw_slice = raw_slice(table, count);
    
    unsafe
    {
        Box::from_raw(raw_slice)
    }
}

impl<'a> IntoIterator for Arguments<'a>
{
    type Item = Argument<'a>;
    type IntoIter = IntoVecIter<Argument<'a>>;
    
    fn into_iter(self) -> IntoVecIter<Argument<'a>>
    {
        let store = ManuallyDrop::new(self);
        
        unsafe
        {
            box_from_arguments(store.table,
                               store.count).into_iter()
        }
    }
}
