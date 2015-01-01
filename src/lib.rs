#![deny(missing_docs)]
#![feature(unsafe_destructor)]
#![unstable]

//! A library for setting current values for stack scope,
//! such as application structure.

pub use current::{ Current, CurrentGuard };

use std::rc::Rc;
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
#[unstable]
pub struct DANGER(());

#[unstable]
impl DANGER {
    /// Creates a new warning sign for functions
    /// that normally works with safe code
    #[unstable]
    pub unsafe fn new() -> DANGER { DANGER(()) }
}

impl<F: 'static, T: SetAt<F>> SetAt<Current<F>> for T {
    #[inline(always)]
    fn set_at(self, obj: &mut Current<F>) {
        self.set_at((*obj).deref_mut());
    }
}

impl<F: 'static, T: ActOn<F, U>, U> ActOn<Current<F>, U> for T {
    #[inline(always)]
    fn act_on(self, obj: &mut Current<F>) -> U {
        self.act_on((*obj).deref_mut())
    }
}

impl<T: 'static, U: GetFrom<T>> GetFrom<Current<T>> for U {
    #[inline(always)]
    fn get_from(obj: &Current<T>) -> U {
        GetFrom::get_from((*obj).deref())
    }
}

impl<'a, T, U: GetFrom<T>> GetFrom<&'a RefCell<T>> for U {
    #[inline(always)]
    fn get_from(obj: & &'a RefCell<T>) -> U {
        GetFrom::get_from(obj.borrow().deref())
    }
}

impl<T, U: GetFrom<T>> GetFrom<Rc<RefCell<T>>> for U {
    #[inline(always)]
    fn get_from(obj: &Rc<RefCell<T>>) -> U {
        GetFrom::get_from(obj.borrow().deref())
    }
}

impl<'a, F, T: SetAt<F>> SetAt<&'a RefCell<F>> for T {
    #[inline(always)]
    fn set_at(self, obj: &mut &'a RefCell<F>) {
        self.set_at(obj.borrow_mut().deref_mut())
    }
}

impl<F, T: SetAt<F>> SetAt<Rc<RefCell<F>>> for T {
    #[inline(always)]
    fn set_at(self, obj: &mut Rc<RefCell<F>>) {
        self.set_at(obj.borrow_mut().deref_mut())
    }
}


impl<'a, F, T: ActOn<F, U>, U> ActOn<&'a RefCell<F>, U> for T {
    #[inline(always)]
    fn act_on(self, obj: &mut &'a RefCell<F>) -> U {
        self.act_on(obj.borrow_mut().deref_mut())
    }
}

impl<F, T: ActOn<F, U>, U> ActOn<Rc<RefCell<F>>, U> for T {
    #[inline(always)]
    fn act_on(self, obj: &mut Rc<RefCell<F>>) -> U {
        self.act_on(obj.borrow_mut().deref_mut())
    }
}

/// Something that can be set at an object.
#[unstable]
pub trait SetAt<F> {
    /// Modify `F` with self.
    fn set_at(self, &mut F);
}

/// Automatically implemented through the `SetAt` trait.
#[unstable]
pub trait Set<T> {
    /// Set value.
    fn set(mut self, val: T) -> Self;

    /// Set value through mutable reference.
    fn set_mut(&mut self, val: T) -> &mut Self;
}

impl<T, U: SetAt<T>> Set<U> for T {
    #[inline(always)]
    fn set(mut self, val: U) -> T {
        val.set_at(&mut self);
        self
    }

    #[inline(always)]
    fn set_mut(&mut self, val: U) -> &mut T {
        val.set_at(self);
        self
    }
}

/// Something that can be retrieved from another object.
#[unstable]
pub trait GetFrom<T> {
    /// Gets value from object.
    fn get_from(obj: &T) -> Self;
}

/// Automatically implemented through the `GetFrom` trait.
#[unstable]
pub trait Get<T> {
    /// Returns new value.
    fn get(&self) -> T;
}

impl<T, U: GetFrom<T>> Get<U> for T {
    #[inline(always)]
    fn get(&self) -> U {
        GetFrom::get_from(self)
    }
}

/// Does something to an object.
#[unstable]
pub trait ActOn<T, U> {
    /// Does something to an object.
    fn act_on(self, &mut T) -> U;
}

/// Automatically implemented through the `ActOn` trait.
#[unstable]
pub trait Action<T, U> {
    /// Does something.
    fn action(&mut self, val: T) -> U;
}

impl<T, U: ActOn<T, V>, V> Action<U, V> for T {
    #[inline(always)]
    fn action(&mut self, val: U) -> V {
        val.act_on(self)
    }
}

