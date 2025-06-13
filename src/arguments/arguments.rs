#[cfg(no_std)]
use alloc::boxed::Box;

#[cfg(no_std)]
use core::{
    ops::Deref
};

#[cfg(not(no_std))]
use std::{
    ops::Deref
};

use crate::Argument;

pub struct Arguments<'a>
{
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
