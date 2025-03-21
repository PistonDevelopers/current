#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![cfg_attr(feature = "unstable", feature(core_intrinsics))]

use std::cell::RefCell;
use std::any::{ TypeId, Any };
use std::collections::HashMap;
use std::collections::hash_map::Entry::{ Occupied, Vacant };
use std::ops::{ Deref, DerefMut };
use std::marker::PhantomData;

// Stores the current pointers for concrete types.
thread_local!(static KEY_CURRENT: RefCell<HashMap<TypeId, usize>>
    = RefCell::new(HashMap::new()));

/// Puts back the previous current pointer.
pub struct CurrentGuard<'a, T> where T: Any {
    _val: &'a mut T,
    old_ptr: Option<usize>
}

#[allow(trivial_casts)]
impl<'a, T> CurrentGuard<'a, T> where T: Any {
    /// Creates a new current guard.
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

impl<'a, T> Drop for CurrentGuard<'a, T> where T: Any {
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
pub struct Current<T>(PhantomData<T>);

impl<T> Current<T> where T: Any {
    /// Creates a new current object
    pub unsafe fn new() -> Current<T> { Current(PhantomData) }

    /// Gets mutable reference to current object.
    /// Requires mutable reference to prevent access to globals in safe code,
    /// and to prevent mutable borrows of same value in scope.
    /// Is unsafe because returned reference inherits lifetime from argument.
    pub unsafe fn current(&mut self) -> Option<&mut T> {
        use std::mem::transmute;
        let id = TypeId::of::<T>();
        let ptr: Option<usize> = KEY_CURRENT.with(|current| {
                current.borrow().get(&id).map(|id| *id)
            });
        let ptr = match ptr { None => { return None; } Some(x) => x };
        Some(unsafe { transmute(ptr as *mut T) })
    }

    /// Unwraps mutable reference to current object,
    /// but with nicer error message.
    pub unsafe fn current_unwrap(&mut self) -> &mut T {
        #[cfg(feature = "unstable")]
        fn report_unstable<T>() -> ! {
            use std::intrinsics::type_name;
            panic!("No current `{}` is set", unsafe { type_name::<T>() });
        }

        #[cfg(not(feature = "unstable"))]
        fn report_unstable<T>() -> ! {
            panic!("No current object is set of this type");
        }

        match unsafe { self.current() } {
            None => report_unstable::<T>(),
            Some(x) => x
        }
    }
}

impl<T> Deref for Current<T> where T: Any {
    type Target = T;

    #[inline(always)]
    fn deref<'a>(&'a self) -> &'a T {
        // This uses a custom version for immutable references
        // to avoid undefined behavior.

        unsafe fn current<'a, T: 'static>(_: &'a T) -> Option<&'a T> {
            use std::mem::transmute;
            let id = TypeId::of::<T>();
            let ptr: Option<usize> = KEY_CURRENT.with(|current| {
                    current.borrow().get(&id).map(|id| *id)
                });
            let ptr = match ptr { None => { return None; } Some(x) => x };
            Some(unsafe { transmute(ptr as *const T) })
        }

        #[cfg(feature = "unstable")]
        fn report_unstable<T>() -> ! {
            use std::intrinsics::type_name;
            panic!("No current `{}` is set", unsafe { type_name::<T>() });
        }

        #[cfg(not(feature = "unstable"))]
        fn report_unstable<T>() -> ! {
            panic!("No current object is set of this type");
        }

        match unsafe { current(self) } {
            None => report_unstable::<T>(),
            Some(x) => x
        }
    }
}

impl<T> DerefMut for Current<T> where T: Any {
    #[inline(always)]
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { self.current_unwrap() }
    }
}
