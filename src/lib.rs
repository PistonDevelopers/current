#![deny(missing_docs)]
#![feature(unsafe_destructor)]

//! A library for setting current values for stack scope,
//! such as application structure.

extern crate modifier;

pub use modifier::{ Modifier, Set };
pub use current::{ Current, CurrentGuard };

mod current;

/// Implemented by types that can be constructed from another value.
pub trait Get<T> {
    /// Returns new value.
    fn get(obj: &T) -> Self;
}

/// Transmutes the current value to a lifetime scope.
/// The scope must not outlive the guard.
/// This is used to work around issues with closure that require move by value.
pub unsafe fn unsafe_current_unwrap<T>(_scope: &()) -> &T {
    Current::with_current_unwrap(|current: &T| {
        use std::mem::transmute;
        transmute(current)
    })
}

/// Sets value on current type, assuming using a `RefCell` to make it safe.
/// Panics if there is no current object of type `T`.
pub fn set<T, U: Modifier<T>>(val: U) {
    use std::cell::RefCell;
    let scope = &();
    let current: &RefCell<T> = unsafe { unsafe_current_unwrap(scope) };
    val.modify(&mut*current.borrow_mut());
}

/// Gets value from current object of type `T`.
/// The returned type must implement the `GetFrom` trait.
pub fn get<T, U: Get<T>>() -> U {
    Current::with_current_unwrap(|current: &T| {
        Get::get(current)
    })
}
