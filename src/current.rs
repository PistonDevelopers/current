use std::cell::RefCell;
use std::intrinsics::TypeId;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{ Occupied, Vacant };

// Stores the current pointers for concrete types.
thread_local!(static KEY_CURRENT: RefCell<HashMap<TypeId, uint>> 
    = RefCell::new(HashMap::new()));

/// Puts back the previous current pointer.
pub struct CurrentGuard<'a, T: 'a> {
    _val: &'a mut T,
    old_ptr: Option<uint>
}

impl<'a, T: 'static> CurrentGuard<'a, T> {
    /// Creates a new current guard.
    pub fn new(val: &mut T) -> CurrentGuard<T> {
        let id = TypeId::of::<T>();
        let ptr = val as *mut T as uint;
        let old_ptr = KEY_CURRENT.with(|current| {
            match current.borrow_mut().entry(id) {
                Occupied(mut entry) => Some(entry.set(ptr)),
                Vacant(entry) => {
                    entry.set(ptr);
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
                        Occupied(mut entry) => { entry.set(old_ptr); }
                        Vacant(entry) => { entry.set(old_ptr); }
                    };
                });
            }
        };
    }
}

/// The current value of a type.
pub struct Current<T>(());

impl<T: 'static> Current<T> {
    /// Creates a new current object
    pub unsafe fn new() -> Current<T> { Current(()) }

    /// Gets mutable reference to current object.
    /// Requires mutable reference to prevent access to globals in safe code,
    /// and to prevent mutable borrows of same value in scope.
    /// Is unsafe because returned reference inherits lifetime from argument.
    pub unsafe fn current(&mut self) -> Option<&mut T> {
        use std::mem::transmute;
        let id = TypeId::of::<T>();
        let ptr: Option<uint> = KEY_CURRENT.with(|current| {
                current.borrow().get(&id).map(|id| *id)
            });
        let ptr = match ptr { None => { return None; } Some(x) => x };
        Some(transmute(ptr as *mut T))
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
