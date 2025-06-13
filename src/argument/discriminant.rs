pub enum Discriminant
{
    Inlined,
    Allocated,
    Borrowed
}

impl Discriminant
{
    pub fn from_owned(inlined: bool) -> Self
    {
        if inlined { Self::Inlined }
        else { Self::Allocated }
    }
    
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
