#[cfg(no_std)]
use alloc::vec::Vec;

#[cfg(no_std)]
use core::{
    any::Any,
    ops::Deref
};

use crate::Argument;

#[cfg(not(no_std))]
use std::{
    any::Any,
    ops::Deref
};

use super::{Arguments, MAX_ARG_COUNT};

/// A structure for creating Arguments.
///
/// This guarantees that the amount of arguments will be no
/// more than MAX_ARG_COUNT.
#[derive(Clone, Debug, Default)]
pub struct ArgumentsBuilder<'a>
{
    /// The inner contents for storing arguments.
    table: Vec<Argument<'a>>
}

impl<'a> Deref for ArgumentsBuilder<'a>
{
    type Target = [Argument<'a>];
    
    #[inline(always)]
    fn deref(&self) -> &[Argument<'a>]
    {
        &self.table
    }
}

impl ArgumentsBuilder<'_>
{
    /// Creates a new instance of ArgumentsBuilder.
    #[inline(always)]
    pub fn new() -> Self
    {
        Self
        {
            table: Vec::new()
        }
    }
    
    /// Creates a new instance of ArgumentsBuilder with a set capacity.
    ///
    /// Even though the cap value could exceed MAX_ARG_COUNT, the output will
    /// always restrict the max capacity to MAX_ARG_COUNT elements.
    #[inline(always)]
    pub fn with_capacity(cap: usize) -> Self
    {
        let cap = if cap <= MAX_ARG_COUNT { cap } else { MAX_ARG_COUNT };
        
        Self
        {
            table: Vec::with_capacity(cap)
        }
    }
    
    /// Determines whether or not ArgumentsBuilder is full.
    ///
    /// Returns true if that is the case.
    #[inline(always)]
    pub fn is_full(&self) -> bool
    {
        self.len() >= MAX_ARG_COUNT
    }
    
    /// A check to see if we can still insert more arguments.
    #[inline(always)]
    fn can_insert_args(&self) -> bool
    {
        self.len() < MAX_ARG_COUNT
    }
    
    /// Prints out the remaining amount of arguments that we are allowed to
    /// add to the structure itself.
    #[inline(always)]
    fn remaining(&self) -> usize
    {
        MAX_ARG_COUNT - self.len()
    }
    
    /// Returns the structure's inner capacity.
    #[inline(always)]
    pub fn capacity(&self) -> usize
    {
        self.table.capacity()
    }
    
    /// Reserves a set amount of elements for the builder itself.
    ///
    /// This does nothing if the inner capacity is at least MAX_ARG_COUNT.
    #[inline(always)]
    pub fn reserve(&mut self, count: usize)
    {
        if self.capacity() < MAX_ARG_COUNT
        {
            let remaining = self.remaining();
            
            let count =
            if count > remaining
            {
                remaining
            } else { count };
            
            self.table.reserve(count);
        }
    }
    
    /// Tries to insert a generic item.
    ///
    /// # Return values
    /// Ok(()): Able to insert the owned item.
    /// Err(owned): The arguments builder is already full.
    #[inline(always)]
    pub fn insert_owned<T>(&mut self, owned: T)
    -> Result<(), T>
    where
        T: Any + Clone
    {
        if self.can_insert_args()
        {
            self.table.push(Argument::new_owned(owned));
            Ok(())
        } else { Err(owned) }
    }
}

impl<'a> ArgumentsBuilder<'a>
{
    /// Removes an argument at the specified index.
    ///
    /// # Return values
    /// Some(arg): There was an argument at said position.
    /// None: There are no arguments at idx.
    #[inline(always)]
    pub fn remove(&mut self, idx: usize) -> Option<Argument<'a>>
    {
        if idx < self.len()
        {
            // This check is done to prevent a panic.
            Some(self.table.remove(idx))
        } else { None }
    }
    
    /// Removes the last element from the builder itself.
    ///
    /// Returns None if the builder is empty.
    #[inline(always)]
    pub fn pop(&mut self) -> Option<Argument<'a>>
    {
        self.table.pop()
    }
    
    /// Tries to insert a generic, borrowed item.
    ///
    /// # Return values
    /// true: We are able to insert the borrowed item itself.
    /// false: The table is already full.
    #[inline(always)]
    pub fn insert_borrowed<T>(&mut self, borrowed: &'a T) -> bool
    where
        T: Any + Clone
    {
        if self.can_insert_args()
        {
            self.table.push(Argument::new_borrowed(borrowed));
            true
        } else { false }
    }
    
    /// Tries to insert an argument that is already in a Argument format.
    ///
    /// # Return values
    /// Ok(()): Able to insert the argument itself.
    /// Err(arg): The builder is already full.
    #[inline(always)]
    pub fn insert_argument(&mut self, arg: Argument<'a>) -> Result<(), Argument<'a>>
    {
        if self.can_insert_args()
        {
            self.table.push(arg);
            Ok(())
        } else { Err(arg) }
    }
    
    /// Tries to extend the builder based around an ExactSizeIterator over Argument<'a>.
    ///
    /// The returned value is the iterator's remaining contents collected into Vec<Argument<'a>>.
    /// 
    /// If said value is not empty, then the builder itself has already reached max capacity.
    #[inline(always)]
    pub fn extend<T>(&mut self, mut args: T) -> Vec<Argument<'a>>
    where
        T: Iterator<Item = Argument<'a>> + ExactSizeIterator
    {
        let remaining = self.remaining();
        
        if remaining != 0
        {
            self.table.extend(args.by_ref().take(remaining));
        }
        
        #[cfg(debug_assertions)]
        {
            if self.is_full()
            {
                assert_eq!(self.len(), MAX_ARG_COUNT);
            }
        }
        
        args.collect()
    }
    
    /// Builds the inner argument table, returning Arguments in exchange.
    #[inline(always)]
    pub fn build(self) -> Arguments<'a>
    {
        assert!(self.len() <= MAX_ARG_COUNT);
        
        match Arguments::from_args(self.table)
        {
            Ok(a) => a,
            _ => unreachable!()
        }
    }
}
