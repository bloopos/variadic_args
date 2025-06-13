use crate::Argument;

use core::fmt::Debug;
use core::any::Any;
use core::ops::Deref;

fn test_borrowed<T>(item: &T)
where
    T: Any + Clone + Eq + Debug
{
    let borrowed = Argument::new_borrowed(item);
    
    let item : T = test_borrowed_state(borrowed);
    
    assert_eq!(item, item.clone());
}

fn test_borrowed_state<'a, T>(borrowed: Argument<'a>) -> T
where
    T: Any + Clone + Eq + Debug
{
    assert!(borrowed.is_borrowed());
    
    assert!(borrowed.is::<T>());
    
    unsafe
    {
        borrowed.downcast_cloned_unchecked::<T>()
    }
}

fn test_owned<T>(item: T)
where
    T: Any + Clone + Eq + Debug
{
    let owned = Argument::new_owned(item.clone());
    
    assert!(owned.is_owned());
    
    {
        let ref_ = owned.as_ref();
        
        let cloned_item : T = test_borrowed_state(ref_);
        
        assert_eq!(item.clone(), cloned_item, "Cloned items do not match!");
    }
    
    assert!(owned.is_owned());

    assert!(owned.deref().is::<T>());
    
    let current_item : T = unsafe { owned.downcast_owned_unchecked() };
    
    assert_eq!(current_item, item);
}

#[test]
fn test_borrowed_zst()
{
    let state = ();
    
    test_borrowed(&state);
}

#[test]
fn test_owned_zst()
{
    let state = ();
    
    test_owned(state);
}

#[test]
fn test_borrowed_i32()
{
    let current = 1_i32;
    test_borrowed(&current);
}

#[test]
fn test_owned_i32()
{
    let current = 1_i32;
    test_owned(current);
}


#[test]
fn test_borrowed_box()
{
    let current = Box::new(1_i32);
    test_borrowed(&current);
}

#[test]
fn test_owned_box()
{
    let current = Box::new(1_i32);
    test_owned(current);
}

#[test]
fn test_borrowed_alloc()
{
    let current = vec!(1_u8; 100);
    test_borrowed(&current);
}


#[test]
fn test_owned_alloc()
{
    let current = vec!(1_u8; 100);
    test_owned(current);
}
