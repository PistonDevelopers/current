current [![Build Status](https://travis-ci.org/PistonDevelopers/current.svg)](https://travis-ci.org/PistonDevelopers/current)
=======

A library for setting current values for stack scope, such as application structure

### How to use it

Declare a function, prefixed with `current_` to indicate that a current object is used:

```Rust
fn current_window() -> Usage<'static, Window> { UseCurrent }
```

When you want to use the current object in a function, you do this:

```Rust
let window = RefCell::new(window); // create a shared reference to the object
let window_guard = window.set_current();
start(); // function that uses the current object.
drop(window_guard);
```

Inside the function where you use the current object, you can call the function and use it as an object:

```Rust
fn start() {
    current_window().set_title("Hello");
    ...
}
```

This works because the `Usage` enum implements `Deref` and `DerefMut` which gets a reference to the current object for the scope it is used.

You can also assign a new value to the current object and get a copy if the object implements `Copy`:

```Rust
// `health` is u32 so we can dereference it to get the value and also assign a new one.
*current_health() = *current_health() - 1;
```

This can also be done with more advanced objects, because Rust calls `drop` and cleans up the old object before it gets replaced with a new one.

Read more in the issue for [Best coding practices with current objects](https://github.com/PistonDevelopers/current/issues/15)

### Why?

When you are calling a normal function, you use order or naming to tell the compiler the relationships between data on the stack:

```Rust
foo(a, b); // use variable `a` and `b` in scope.
```

However, you can use any convention to describe relationships, not just order or naming.
In this library we have developed a safe convention that uses concrete types.

```Rust
foo::<A>(b); // use current value of type `A` and variable `b` in scope.
```

Each concrete type can have a "current" value,
which is accessible to all functions that knows about the type.
Immutable references are supported by default,
but if you want mutability, you can use [RefCell](http://doc.rust-lang.org/std/cell/struct.RefCell.html).

```Rust
let a = RefCell::new(a); // RefCell prevents multiple mutable references at run time.
let a_guard = a.set_current();

foo::<A>(b); // use type `A` and variable `b` in scope.

drop(a_guard);

fn foo<T>(b: &mut B) {
    Current::with_current(|a: &RefCell<T>| {
        let a = a.borrow_mut();
        *a = ...
    });
}
```

In game programming, there are many kinds of "current" values:

* The current window
* The current device
* The current sound driver
* The current player object

By setting these up as "current" values, you don't have to pass them around to each method.
For example, you can write code like this:

```Rust
e.press(|button| {
    if button == SHOOT {
        current_gun().shoot(current_player().aim);
    }
});
```

This makes it easier to decouple data from each other in the application structure.

The major motivation for this library is to have a convention that works across libraries,
such that two people can share code by depending on the same library,
without knowing what the other person is doing.
This is important for building higher level game libraries.

