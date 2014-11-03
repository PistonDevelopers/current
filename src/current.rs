
use std::intrinsics::TypeId;
use std::collections::HashMap;
use std::collections::hashmap::{ Occupied, Vacant };

// Stores the current pointers for concrete types.
local_data_key!(key_current: HashMap<TypeId, uint>)

/// Puts back the previous current pointer.
pub struct CurrentGuard<'a, T: 'a> {
    _val: &'a T,
    old_ptr: Option<uint>
}

#[unsafe_destructor]
impl<'a, T: 'static> Drop for CurrentGuard<'a, T> {
    fn drop(&mut self) {
        let id = TypeId::of::<T>();
        let mut current = key_current.replace(None).unwrap();
        match self.old_ptr {
            None => {
                current.remove(&id);
                return;
            }
            Some(old_ptr) => {
                match current.entry(id) {
                    Occupied(mut entry) => { entry.set(old_ptr); }
                    Vacant(entry) => { entry.set(old_ptr); }
                };
            }
        };
        key_current.replace(Some(current));
    }
}

/// Implemented by all concrete types to define a current value for a scope.
pub trait Current {
    /// Sets current mutable borrow for this concrete type.
    fn set_current<'a>(&'a self) -> CurrentGuard<'a, Self>;
    /// Calls closure if the current value is set.
    fn with_current<U>(f: |&Self| -> U) -> Option<U>;
    /// Calls closure if the current value is set.
    /// Gives a nicer error message of the expected type.
    fn with_current_unwrap<U>(f: |&Self| -> U) -> U;
}

impl<T: 'static> Current for T {
    fn set_current(&self) -> CurrentGuard<T> {
        let id = TypeId::of::<T>();
        let ptr = self as *const T as uint;
        let current = key_current.replace(None);
        let mut current = match current {
            None => HashMap::new(),
            Some(current) => current
        };
        let old_ptr = match current.entry(id) {
            Occupied(mut entry) => Some(entry.set(ptr)),
            Vacant(entry) => {
                entry.set(ptr);
                None
            }
        };
        key_current.replace(Some(current));
        CurrentGuard { old_ptr: old_ptr, _val: self }
    }

    fn with_current<U>(f: |&T| -> U) -> Option<U> {
        use std::mem::transmute;
        let id = TypeId::of::<T>();
        let current = match key_current.replace(None) {
            None => { return None; }
            Some(current) => current
        };
        let ptr = match current.find(&id) {
            None => { return None; }
            Some(x) => *x
        };
        key_current.replace(Some(current));
        Some(f(unsafe { transmute(ptr as *const T) }))
    }

    fn with_current_unwrap<U>(f: |&T| -> U) -> U {
        match Current::with_current(f) {
            None => {
                use std::intrinsics::get_tydesc;
                let name = unsafe { (*get_tydesc::<T>()).name };
                panic!("No current `{}` is set", name);
            }
            Some(x) => x
        }
    }
}
