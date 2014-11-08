#![license = "MIT"]
#![deny(missing_docs)]
#![feature(unsafe_destructor)]

//! A library for setting current values for stack scope,
//! such as application structure.

pub use current::{ Current, CurrentGuard };

use std::cell::RefCell;

mod current;

/// Specifies whether to use a shared reference,
/// or the current object of that type.
pub enum Usage<'a, T: 'a> {
    /// Use a shared reference.
    Use(&'a RefCell<T>),
    /// Use the current object.
    UseCurrent,
}

impl<'a, T: 'a> Usage<'a, T> {
    /// Calls closure with a shared reference to used object.
    pub fn with<U>(&self, f: |&RefCell<T>| -> U) -> Option<U> {
        match *self {
            Use(val) => Some(f(val)),
            UseCurrent => Current::with_current(f),
        }
    }

    /// Calls closure with a shared reference to used object.
    /// Gives a nicer error message if current object is not set.
    pub fn with_unwrap<U>(&self, f: |&RefCell<T>| -> U) -> U {
        match *self {
            Use(val) => f(val),
            UseCurrent => Current::with_current_unwrap(f),
        }
    }

    /// Unwraps to the scope of usage.
    /// Should only be called when the usage object or returned reference
    /// does not outlives the guard of the current object.
    #[inline(always)]
    pub unsafe fn unsafe_unwrap(&self) -> &RefCell<T> {
        match *self {
            Use(val) => val,
            UseCurrent => {
                Current::with_current_unwrap(|current: &RefCell<T>| {
                    use std::mem::transmute;
                    transmute(current)
                })
            }
        }
    }
}

impl<'a, T: Get<U>, U> Get<U> for Usage<'a, T> {
    #[inline(always)]
    fn get(&self) -> U {
        self.with_unwrap(|val: &RefCell<T>| {
            val.borrow().deref().get()
        })
    }
}

impl<'a, F, T: Modifier<F>> Modifier<Usage<'a, F>> for T {
    #[inline(always)]
    fn modify(self, obj: &mut Usage<'a, F>) {
        unsafe {
            let val: &RefCell<F> = obj.unsafe_unwrap();
            self.modify(val.borrow_mut().deref_mut())
        }
    }
}

impl<T: Get<U>, U> Get<U> for RefCell<T> {
    #[inline(always)]
    fn get(&self) -> U {
        self.borrow().deref().get()
    }
}

impl<F, T: Modifier<F>> Modifier<RefCell<F>> for T {
    #[inline(always)]
    fn modify(self, obj: &mut RefCell<F>) {
        self.modify(obj.borrow_mut().deref_mut())
    }
}

/// Allows use of the implemented type as an argument to Set::set.
///
/// This allows types to be used for ad-hoc overloading of Set::set
/// to perform complex updates to the parameter of Modifier.
pub trait Modifier<F> {
    /// Modify `F` with self.
    fn modify(self, &mut F);
}

/// A blanket trait providing the set and set_mut methods for all types.
pub trait Set<M: Modifier<Self>> {
    /// Modify self using the provided modifier.
    #[inline(always)]
    fn set(mut self, modifier: M) -> Self {
        modifier.modify(&mut self);
        self
    }

    /// Modify self through a mutable reference with the provided modifier.
    #[inline(always)]
    fn set_mut(&mut self, modifier: M) -> &mut Self {
        modifier.modify(self);
        self
    }
}

impl<T, U: Modifier<T>> Set<U> for T {}

/// Implemented by types that can be constructed from another value.
pub trait Get<T> {
    /// Returns new value.
    fn get(&self) -> T;
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
    let scope = &();
    let current: &RefCell<T> = unsafe { unsafe_current_unwrap(scope) };
    val.modify(&mut*current.borrow_mut());
}

/// Gets value from current object of type `T`.
/// The returned type must implement the `GetFrom` trait.
pub fn get<T: Get<U>, U>() -> U {
    Current::with_current_unwrap(|current: &RefCell<T>| {
        match current.try_borrow() {
            Some(val) => val.get(),
            None => {
                use std::intrinsics::get_tydesc;
                let name = unsafe { (*get_tydesc::<T>()).name };
                panic!("The current `{}` is already in use", name);
            }
        }
    })
}
