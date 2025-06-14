#[cfg(no_std)]
use alloc::{
    boxed::Box,
    vec::IntoIter as VecIntoIter,
    vec::Vec
};

#[cfg(no_std)]
use core::{
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut}
};

#[cfg(not(no_std))]
use std::{
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
    vec::IntoIter as VecIntoIter
};

use super::MAX_ARG_COUNT;

use crate::Argument;

/// A container for storing a set of arguments.
///
/// While the inner storage's size is fixed, the storage
/// can be accessed mutably.
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Arguments<'a>
{
    /// The inner table for storing a slice of arguments.
    table: Box<[Argument<'a>]>
}


impl<'a> Deref for Arguments<'a>
{
    type Target = [Argument<'a>];
    
    fn deref(&self) -> &[Argument<'a>]
    {
        &self.table
    }
}


impl<'a> DerefMut for Arguments<'a>
{
    fn deref_mut(&mut self) -> &mut [Argument<'a>]
    {
        &mut self.table
    }
}


impl<'a> Arguments<'a>
{
    /// Imports a set of arguments from a boxed slice.
    ///
    /// # Return values
    /// Ok(Self): Argument count is no more than MAX_ARG_COUNT.
    /// Err(args): Argument count is greater than MAX_ARG_COUNT.
    pub fn from_boxed_args(args: Box<[Argument<'a>]>) -> Result<Self, Box<[Argument<'a>]>>
    {
        if args.len() <= MAX_ARG_COUNT
        {
            Ok
            (
                Self
                {
                    table: args
                }
            )
        }
        else { Err(args) }
    }
    
    
    /// Imports a set of arguments from Vec<Argument<'a>>.
    ///
    /// # Return values
    /// Refer to Arguments::from_boxed_args for information about return values.
    pub fn from_args(args: Vec<Argument<'a>>) -> Result<Self, Vec<Argument<'a>>>
    {
        if args.len() <= MAX_ARG_COUNT
        {
            Ok
            (
                Self
                {
                    table: args.into_boxed_slice()
                }
            )
        }
        else { Err(args) }
    }
    
    
    /// Imports a set of arguments from an iterator over Argument items.
    ///
    /// While it does enable support for other iterators, the iterator itself must
    /// implement ExactSizeIterator.
    ///
    /// # Return values
    /// Ok(Self): Arg count is no more than MAX_ARG_COUNT.
    /// Err(e): Arg count is greater than MAX_ARG_COUNT. The error value is the iterator
    /// collected into Vec<Argument<'a>>.
    pub fn from_iter<T>(args: T) -> Result<Self, Vec<Argument<'a>>>
    where
        T: Iterator<Item = Argument<'a>> + ExactSizeIterator
    {
        if args.len() <= MAX_ARG_COUNT
        {
            Ok(Self
            {
                table: args.collect()
            }
            )
        }
        else
        {
            Err(args.collect())
        }
    }
    
    
    /// Iterates over a borrowed set of arguments.
    pub fn iter(&self) -> Iter<'_, Argument<'a>>
    {
        self.table.iter()
    }
    
    
    /// Iterates over a mutable set of arguments.
    pub fn iter_mut(&mut self) -> IterMut<'_, Argument<'a>>
    {
        self.table.iter_mut()
    }
}


impl<'a> IntoIterator for Arguments<'a>
{
    type Item = Argument<'a>;
    type IntoIter = VecIntoIter<Argument<'a>>;
    
    fn into_iter(self) -> VecIntoIter<Argument<'a>>
    {
        self.table.into_iter()
    }
}
