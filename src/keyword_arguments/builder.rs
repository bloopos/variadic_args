use super::{Error, ErrorKind, Key, key_ident::validate_key, keyed_args::KeywordArguments};
use crate::MAX_ARG_COUNT;

#[cfg(no_std)]
use alloc::collections::{
    BTreeMap,
    btree_map::{Iter, Keys}
};

#[cfg(no_std)]
use core::any::Any;

#[cfg(not(no_std))]
use std::{
    any::Any,
    collections::{
        BTreeMap,
        btree_map::{Iter, Keys}
    }
};

use crate::Argument;

pub struct KeywordArgumentsBuilder<'a>
{
    table: BTreeMap<Key, Argument<'a>>
}


impl KeywordArgumentsBuilder<'_>
{
    pub fn len(&self) -> usize
    {
        self.table.len()
    }

    pub fn is_full(&self) -> bool
    {
        self.len() >= MAX_ARG_COUNT
    }

    pub fn contains_key(&self, key: &str) -> Option<bool>
    {
        if validate_key(key).is_ok()
        {
            Some(unsafe { self.contains_key_unchecked(key) })
        } else { None }
    }

    unsafe fn contains_key_unchecked(&self, key: &str) -> bool
    {
        self.table.contains_key(key)
    }

    fn validate_key<T>(&self, key: Key, value: T) -> Result<(Key, T), Error<T>>
    {
        if let Err(kind) = validate_key(&key)
        {
            let invalid = Error::new(kind, key, value);

            return Err(invalid);
        }

        if self.is_full()
        {
            let full = Error::new(ErrorKind::MaxArguments, key, value);

            return Err(full);
        }
        else if unsafe { self.contains_key_unchecked(&key) }
        {
            let exists = Error::new(ErrorKind::KeyExists, key, value);

            return Err(exists);
        }

        Ok((key, value))
    }

    pub fn insert_owned<K, T>(&mut self, key: K, value: T) -> Result<(), Error<T>>
    where
        K: Into<Key>,
        T: Any + Clone
    {
        let (key, val) = self.validate_key(key.into(), value)?;

        let value = Argument::new_owned(val);

        unsafe
        {
            self.insert_raw_unchecked(key, value);
        }

        Ok(())
    }

    pub fn keys(&self) -> Keys<'_, Key, Argument<'_>>
    {
        self.table.keys()
    }
}

impl<'a> KeywordArgumentsBuilder<'a>
{
    unsafe fn insert_raw_unchecked(&mut self, key: Key, value: Argument<'a>)
    {
        #[cfg(debug_assertions)]
        {
            assert!(self.table.insert(key, value).is_none());
        }
        #[cfg(not(debug_assertions))]
        {
            let _ = self.table.insert(key, value);
        }
    }

    pub fn remove(&mut self, key: &str) -> Result<Argument<'a>, ErrorKind>
    {
        validate_key(key)?;

        match self.table.remove(key)
        {
            Some(v) => Ok(v),
            None => Err(ErrorKind::KeyDoesNotExist)
        }
    }

    pub fn insert_borrowed<K, T>(&mut self, key: K, value: &'a T) -> Result<(), Error<()>>
    where
        K: Into<Key>,
        T: Any + Clone
    {
        let (key, _) = self.validate_key(key.into(), ())?;

        unsafe
        {
            self.insert_raw_unchecked(key, Argument::new_borrowed(value));
        }

        Ok(())
    }

    pub fn insert_argument<K>(&mut self, key: K, arg: Argument<'a>) -> Result<(), Error<Argument<'a>>>
    where
        K: Into<Key>
    {
        let (key, value) = self.validate_key(key.into(), arg)?;

        unsafe
        {
            self.insert_raw_unchecked(key, value);
        }

        Ok(())
    }

    pub fn iter(&self) -> Iter<'_, Key, Argument<'a>>
    {
        self.table.iter()
    }

    pub fn build(self) -> KeywordArguments<'a>
    {
        KeywordArguments::from_builder(self.table)
    }
}
