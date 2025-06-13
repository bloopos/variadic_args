#[cfg(no_std)]
use alloc::vec::Vec;

#[cfg(no_std)]
use core::{
    ops::Deref
};

#[cfg(not(no_std))]
use std::{
    ops::Deref
};

use crate::Argument;

pub struct ArgumentsBuilder<'a>
{
    table: Vec<Argument<'a>>
}

impl<'a> Deref for ArgumentsBuilder<'a>
{
    type Target = [Argument<'a>];
    
    fn deref(&self) -> &[Argument<'a>]
    {
        &self.table
    }
}
