#![deny(missing_docs)]
#![feature(unsafe_destructor)]
#![feature(core, std_misc)]
#![unstable]

//! A library for setting current values for stack scope,
//! such as application structure.

use std::cell::RefCell;
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{ Occupied, Vacant };
use std::ops::{ Deref, DerefMut };
use std::marker::PhantomData;

// Stores the current pointers for concrete types.
thread_local!(static KEY_CURRENT: RefCell<HashMap<TypeId, usize>> 
    = RefCell::new(HashMap::new()));

/// Puts back the previous current pointer.
#[unstable]
pub struct CurrentGuard<'a, T: 'a> {
    _val: &'a mut T,
    old_ptr: Option<usize>
}

#[unstable]
impl<'a, T: 'static> CurrentGuard<'a, T> {
    /// Creates a new current guard.
    #[unstable]
    pub fn new(val: &mut T) -> CurrentGuard<T> {
        let id = TypeId::of::<T>();
        let ptr = val as *mut T as usize;
        let old_ptr = KEY_CURRENT.with(|current| {
            match current.borrow_mut().entry(id) {
                Occupied(mut entry) => Some(entry.insert(ptr)),
                Vacant(entry) => {
                    entry.insert(ptr);
                    None
                }
            }
        });
        CurrentGuard { old_ptr: old_ptr, _val: val }
    }
}

#[unsafe_destructor]
impl<'a, T: 'static> Drop for CurrentGuard<'a, T> {
    fn drop(&mut self) {
        let id = TypeId::of::<T>();
        match self.old_ptr {
            None => {
                KEY_CURRENT.with(|current| {
                    current.borrow_mut().remove(&id);
                });
                return;
            }
            Some(old_ptr) => {
                KEY_CURRENT.with(|current| {
                    match current.borrow_mut().entry(id) {
                        Occupied(mut entry) => { entry.insert(old_ptr); }
                        Vacant(entry) => { entry.insert(old_ptr); }
                    };
                });
            }
        };
    }
}

/// The current value of a type.
#[unstable]
pub struct Current<T>(PhantomData<T>);

#[unstable]
impl<T: 'static> Current<T> {
    /// Creates a new current object
    #[unstable]
    pub unsafe fn new() -> Current<T> { Current(PhantomData) }

    /// Gets mutable reference to current object.
    /// Requires mutable reference to prevent access to globals in safe code,
    /// and to prevent mutable borrows of same value in scope.
    /// Is unsafe because returned reference inherits lifetime from argument.
    #[unstable]
    pub unsafe fn current(&mut self) -> Option<&mut T> {
        use std::mem::transmute;
        let id = TypeId::of::<T>();
        let ptr: Option<usize> = KEY_CURRENT.with(|current| {
                current.borrow().get(&id).map(|id| *id)
            });
        let ptr = match ptr { None => { return None; } Some(x) => x };
        Some(transmute(ptr as *mut T))
    }

    /// Unwraps mutable reference to current object,
    /// but with nicer error message.
    #[unstable]
    pub unsafe fn current_unwrap(&mut self) -> &mut T {
        match self.current() {
            None => {
                use std::intrinsics::type_name;
                panic!("No current `{}` is set", type_name::<T>());
            }
            Some(x) => x
        }
    }
}

impl<T: 'static> Deref for Current<T> {
    type Target = T;

    #[inline(always)]
    fn deref<'a>(&'a self) -> &'a T {
        use std::mem::transmute;
        unsafe {
            // Current does not contain anything,
            // so it is safe to transmute to mutable.
            transmute::<_, &'a mut Current<T>>(self).current_unwrap()
        }
    }
}

impl<T: 'static> DerefMut for Current<T> {
    #[inline(always)]
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { self.current_unwrap() }
    }
}
