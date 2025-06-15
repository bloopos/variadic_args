use super::ErrorKind;

pub(super) fn validate_key(key: &str) -> Result<(), ErrorKind>
{
    let bytes = key.as_bytes();

    if !bytes.is_ascii()
    {
        return Err(ErrorKind::NonASCIIKey);
    }

    if bytes.is_empty()
    {
        return Err(ErrorKind::EmptyKey);
    }

    validate_key_ident(bytes)
}

fn validate_key_ident(key_bytes: &[u8]) -> Result<(), ErrorKind>
{
    if key_bytes.iter().copied().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        Ok(())
    }
    else
    {
        Err(ErrorKind::InvalidKeyName)
    }
}
