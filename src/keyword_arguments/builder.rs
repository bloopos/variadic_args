#[cfg(no_std)]
use alloc::collections::BTreeMap;

#[cfg(not(no_std))]
use std::collections::BTreeMap;

use crate::Argument;

use super::Key;

pub struct KeywordArgumentsBuilder<'a>
{
    table: BTreeMap<Key, Argument<'a>>
}
