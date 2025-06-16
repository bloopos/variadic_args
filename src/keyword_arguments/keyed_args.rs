use crate::Argument;

use super::Key;

#[cfg(no_std)]
use alloc::collections::BTreeMap;

#[cfg(not(no_std))]
use std::collections::BTreeMap;

pub struct KeywordArguments<'a>
{
    table: BTreeMap<Key, Argument<'a>>
}

impl<'a> KeywordArguments<'a>
{
    pub(crate) fn from_builder(table: BTreeMap<Key, Argument<'a>>) -> Self
    {
        Self
        {
            table
        }
    }
}
