use std::cell::RefCell;
use std::thread_local::scoped::Key;

/// Convenience method for setting up current objects for a scope
///
/// Requires scoped thread local objects
///
/// ```ignore
/// scoped_thread_local!(static FOO: RefCell<Foo>)
/// scoped_thread_local!(static BAR: RefCell<Bar>)
/// scoped_thread_local!(static BAZ: RefCell<Baz>)
///
/// fn main() {
///     let foo = ...;
///     let bar = ...;
///     let baz = ...;
///
///     let foo = RefCell::new(foo);
///     let bar = RefCell::new(bar);
///     let baz = RefCell::new(baz);
///     current! {
///         FOO: foo,
///         BAR: bar,
///         BAZ: baz
///         || start()
///     }
/// }
/// ```
#[macro_export]
macro_rules! current {
    (|| $f:expr) => {{
        $f
    }};
    ($HEAD:ident: $head:ident $(,$TAIL:ident: $tail:ident)* || $f:expr) => {{
        $HEAD.set(&$head, || {
            current!{ $($TAIL: $tail),* || $f }
        })
    }};
}

/// Unsafe convenience wrapper for scoped thread local shared objects
///
/// This should be used with caution since calling methods
/// that take `&mut self` risks creating multiple mutable borrows.
///
/// Recommended way of using it is to create an unsafe function
/// and never assign to a variable outside an outside block.
///
/// ```ignore
/// scoped_thread_local!(pub static FOO: RefCell<Foo>)
///
/// pub unsafe fn current_foo() -> Current<Foo> { Current::new(&FOO) }
///
/// unsafe {
///     current_foo().bar();
/// }
/// ```
///
/// Can not be moved across tasks to prevent logic errors
/// when attempting to move an object that uses a current object.
pub struct Current<T: 'static> {
    /// The scoped thread local key containing object.
    key: &'static Key<RefCell<T>>,
    marker: ::std::kinds::marker::NoSync,
}

impl<T: 'static> Current<T> {
    /// Creates a new current object object.
    pub unsafe fn new(key: &'static Key<RefCell<T>>) -> Current<T> {
        Current { key: key, marker: ::std::kinds::marker::NoSync }
    }

    /// Gets mutable reference to current object.
    /// Requires mutable reference to prevent access to globals in safe code,
    /// and to prevent mutable borrows of same value in scope.
    /// Is unsafe because returned reference inherits lifetime from argument.
    pub unsafe fn current(&mut self) -> Option<&mut T> {
        use std::mem::transmute;
        if self.key.is_set() {
            self.key.with(|key| {
                Some(transmute(key.borrow_mut().deref_mut()))
            })
        } else {
            None
        }
    }

    /// Unwraps mutable reference to current object,
    /// but with nicer error message.
    pub unsafe fn current_unwrap(&mut self) -> &mut T {
        match self.current() {
            None => {
                use std::intrinsics::get_tydesc;
                let name = (*get_tydesc::<T>()).name;
                panic!("No current `{}` is set", name);
            }
            Some(x) => x
        }
    }
}

impl<T: 'static> Deref<T> for Current<T> {
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

impl<T: 'static> DerefMut<T> for Current<T> {
    #[inline(always)]
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { self.current_unwrap() }
    }
}

