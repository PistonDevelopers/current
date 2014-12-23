#![deny(missing_docs)]
#![feature(unsafe_destructor)]

//! A library for setting current values for stack scope,
//! such as application structure.

pub use current::{ Current, CurrentGuard };

use std::cell::RefCell;

mod current;

/// Used as warning sign for functions
/// that under normal circumstances works with safe code,
/// but in some edge cases are unsafe.
///
/// 1. The function MUST take `DANGER` as first argument.
///
/// 2. The function MUST take ownership of the value.
///
/// 3. The function docs MUST include a notice `### DANGER`
/// describing the edge case.
///
/// It can not be copied, so an unsafe block is required
/// whenever it is used.
#[allow(missing_copy_implementations)]
pub struct DANGER(());

impl DANGER {
    /// Creates a new warning sign for functions
    /// that normally works with safe code
    pub unsafe fn new() -> DANGER { DANGER(()) }
}

impl<F: 'static, T: Modifier<F>> Modifier<Current<F>> for T {
    #[inline(always)]
    fn modify(self, obj: &mut Current<F>) {
        self.modify((*obj).deref_mut());
    }
}

impl<T: 'static + Get<U>, U> Get<U> for Current<T> {
    #[inline(always)]
    fn get(&self) -> U {
        (*self).deref().get()
    }
}

impl<'a, T: Get<U>, U> Get<U> for &'a RefCell<T> {
    #[inline(always)]
    fn get(&self) -> U {
        self.borrow().deref().get()
    }
}

impl<'a, F, T: Modifier<F>> Modifier<&'a RefCell<F>> for T {
    #[inline(always)]
    fn modify(self, obj: &mut &'a RefCell<F>) {
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
pub trait Set<M> {
    /// Modify self using the provided modifier.
    fn set(mut self, modifier: M) -> Self;

    /// Modify self through a mutable reference with the provided modifier.
    fn set_mut(&mut self, modifier: M) -> &mut Self;
}

impl<T, U: Modifier<T>> Set<U> for T {
    #[inline(always)]
    fn set(mut self, modifier: U) -> T {
        modifier.modify(&mut self);
        self
    }

    #[inline(always)]
    fn set_mut(&mut self, modifier: U) -> &mut T {
        modifier.modify(self);
        self
    }
}

/// Implemented by types that can be constructed from another value.
pub trait Get<T> {
    /// Returns new value.
    fn get(&self) -> T;
}
