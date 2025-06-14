#[cfg(no_std)]
use alloc::boxed::Box;

use super::{
    discriminant::Discriminant,
    inlined::Inlined,
    VariantHandle
};

/// A raw alias for the contents inside an owned argument.
pub enum BoxedArgument
{
    /// The contents have been allocated.
    Allocated(Box<dyn VariantHandle>),
    /// The contents have been inlined.
    Inlined(Inlined)
}

impl BoxedArgument
{
    /// Acquires the inner pointer from an owned argument.
    ///
    /// # SAFETY
    /// The following must be guaranteed for defined behavior.
    ///  * The pointer must be owned.
    ///  * The pointer must point to an actual OwnedArgument.
    #[inline(always)]
    pub unsafe fn from_owned(store: *mut dyn VariantHandle,
                             discriminant: Discriminant) -> Self
    {
        debug_assert!(!store.is_null());

        // Safety:
        //
        // As long as the discriminant matches correctly,
        // the following are assumed to be said state.
        match discriminant
        {
            Discriminant::Inlined =>
            {
                let inlined = Inlined::from(store);

                Self::Inlined(inlined)
            }
            Discriminant::Allocated =>
            {
                let allocated = unsafe { Box::from_raw(store) };

                Self::Allocated(allocated)
            }
            _ => unreachable!()
        }
    }
}
