current [![Build Status](https://travis-ci.org/PistonDevelopers/current.svg)](https://travis-ci.org/PistonDevelopers/current)
=======

A library for setting current values for stack scope, such as application structure

Example project: [Sea Birds' Breakfast](https://github.com/bvssvni/ld31)

### How to use it

Declare an unsafe function, prefixed with `current_` to indicate that a current object is used:

```Rust
unsafe fn current_window() -> Current<Window> { Current::new() }
```

When you want to use the current object in a function, you do this:

```Rust
let window_guard = CurrentGuard::new(&mut window); // make window current object
start(); // function that uses the current object.
drop(window_guard); // put back old current object
```

Inside the function where you use the current object, you can call the function and use it as an object:

```Rust
fn start() {
    unsafe { current_window() }.set_title("Hello");
    ...
}
```

This works because the `Current` implements `Deref` and `DerefMut` which gets a reference to the current object for the scope it is used.

You can also assign a new value to the current object and get a copy if the object implements `Copy`:

```Rust
// `health` is u32 so we can dereference it to get the value and also assign a new one.
*current_health() = *current_health() - 1;
```

This can also be done with more advanced objects, because Rust calls `drop` and cleans up the old object before it gets replaced with a new one.

Read more in the issue for [Best coding practices with current objects](https://github.com/PistonDevelopers/current/issues/15)

### Motivation

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
        unsafe { current_gun() }.shoot(unsafe { current_player() }.aim);
    }
});
```

This makes it easier to decouple data from each other in the application structure.

The major motivation for this library is to have a convention that works across libraries.

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)
