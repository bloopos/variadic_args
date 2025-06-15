use super::Key;

#[cfg(no_std)]
use core::{
    fmt,
    error::Error as RustError
};

#[cfg(not(no_std))]
use std::{
    fmt,
    error::Error as RustError
};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ErrorKind
{
    NonASCIIKey,
    InvalidKeyName,
    EmptyKey,
    KeyExists,
    KeyDoesNotExist,
    MaxArguments
}

pub struct Error<T>
{
    inner: Option<(Key, T)>,
    kind: ErrorKind
}

impl<T> Error<T>
{
    fn key_name(&self) -> &Key
    {
        match self.inner
        {
            Some((ref key, _)) => key,
            None => unreachable!()
        }
    }
    
    pub(crate) fn new(kind: ErrorKind,
                      key: Key,
                      value: T)
    -> Self
    {
        Self
        {
            inner: Some((key, value)),
            kind
        }
    }
    
    pub fn error_kind(&self) -> ErrorKind
    {
        self.kind
    }
    
    pub fn into_inner(self) -> (Key, T)
    {
        match self.inner
        {
            Some(inner) => inner,
            None => unreachable!()
        }
    }
}

impl<T> fmt::Debug for Error<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let key = self.key_name();
        
        f.debug_struct("Error")
         .field("kind", &self.kind)
         .field("key", key)
         .finish()
    }
}

impl<T> fmt::Display for Error<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let key = self.key_name();
        
        match self.kind
        {
            ErrorKind::EmptyKey => write!(f, "The key provided is empty!"),
            ErrorKind::KeyExists => write!(f, "Key {} already exists!", key),
            ErrorKind::NonASCIIKey => write!(f, "The key provided, {}, has invalid ASCII bytes!", key),
            ErrorKind::InvalidKeyName => write!(f, "The key provided, {}, is not a valid key identifier!", key),
            ErrorKind::MaxArguments => write!(f, "The keyword arguments builder has already reached the maximum argument count!"),
            ErrorKind::KeyDoesNotExist => write!(f, "The keyword arguments does not contain the provided key, {}.", key)
        }
    }
}

impl<T> RustError for Error<T> { }
