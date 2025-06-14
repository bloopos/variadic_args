use crate::Argument;

use super::Key;

#[cfg(no_std)]
use core::mem::MaybeUninit;

#[cfg(not(no_std))]
use std::mem::MaybeUninit;

pub struct KeywordArguments<'a>
{
    working: *mut (MaybeUninit<Key>, Argument<'a>),
    allocated_size: usize,
    left: u16
}
