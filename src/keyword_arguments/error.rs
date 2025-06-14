use super::Key;

#[repr(u8)]
pub enum ErrorKind
{
    NonASCIIKey,
    InvalidKeyName,
    EmptyKey,
    KeyExists
}

pub struct Error<T>
{
    inner: Option<(Key, T)>,
    kind: ErrorKind
}
