/// A discriminant flag.
///
/// This is meant to simplify the code flow for Argument status.
#[repr(u8)]
pub enum Discriminant
{
    /// The storage pointer is owned and inlined.
    Inlined,
    /// The storage pointer is owned and allocated.
    Allocated,
    /// The storage pointer is borrowed.
    Borrowed
}

impl Discriminant
{
    /// Creates a discriminant from an inlined flag.
    ///
    /// By definition, this should return either Inlined or Allocated.
    #[inline(always)]
    pub fn from_owned(inlined: bool) -> Self
    {
        if inlined { Self::Inlined }
        else { Self::Allocated }
    }
    
    /// Creates a new discriminant based around the following flags:
    ///
    /// owned: For determining if the pointer is owned or not,
    /// inlined: For determining the pointer's inline status. This value
    /// gets ignored if owned is false.
    #[inline(always)]
    pub fn
    from_info((inlined, owned): (bool, bool)) -> Self
    {
        match (inlined, owned)
        {
            (_, false) => Self::Borrowed,
            (true, true) => Self::Inlined,
            (false, true) => Self::Allocated
        }
    }
}
