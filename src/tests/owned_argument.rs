use crate::OwnedArgument;

use core::sync::atomic::{AtomicU8, Ordering};
use core::any::Any;

fn
test_owned<T>(item: T)
where
    T: Any + Clone
{
    let owned = OwnedArgument::new(item);
    
    if size_of::<T>() <= size_of::<*const ()>()
    {
        assert!(owned.is_inlined())
    }
    else
    {
        assert!(!owned.is_inlined())
    }
    
    match owned.downcast_owned::<T>()
    {
        Ok(o) => drop(o),
        Err(e) =>
        {
            assert!(e.is_type::<T>());
            unreachable!()
        }
    }
}

#[test]
fn test_zst()
{
    test_owned(())
}

#[test]
fn test_unique_zst()
{
    static NUM : AtomicU8 = AtomicU8::new(1);
    
    #[derive(Clone)]
    struct ZstSample(());

    impl Drop for ZstSample
    {
        fn drop(&mut self)
        {
            assert_ne!(NUM.fetch_sub(1, Ordering::Relaxed), 0);
        }
    }
    
    test_owned(ZstSample(()));
        
    assert_eq!(NUM.load(Ordering::Relaxed), 0);
}

#[test]
fn test_boxed()
{
    test_owned(Box::new(1_i32));
}

#[test]
fn test_unique_boxed()
{
    static NUM : AtomicU8 = AtomicU8::new(1);
    
    #[derive(Clone)]
    struct BoxedSample(());
    
    impl Drop for BoxedSample
    {
        fn drop(&mut self)
        {
            assert_ne!(NUM.fetch_sub(1, Ordering::Relaxed), 0);
        }
    }
    
    test_owned(Box::new(BoxedSample(())));
    
    assert_eq!(NUM.load(Ordering::Relaxed), 0);
}

#[test]
fn test_alloc()
{
    test_owned(vec!(1_u8; 100));
}

#[test]
fn test_unique_alloc()
{
    static NUM : AtomicU8 = AtomicU8::new(1);
    
    #[derive(Clone)]
    struct AllocSample(Vec<u8>);
    
    impl Drop for AllocSample
    {
        fn drop(&mut self)
        {
            assert_ne!(NUM.fetch_sub(1, Ordering::Relaxed), 0);
        }
    }
    
    test_owned(AllocSample(vec!(1_u8; 100)));
    
    assert_eq!(NUM.load(Ordering::Relaxed), 0);
}
