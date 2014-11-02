current [![Build Status](https://travis-ci.org/PistonDevelopers/current.svg)](https://travis-ci.org/PistonDevelopers/current)
=======

A library for setting current values for stack scope, such as application structure

### Why?

When you are calling a normal function, you use order or naming to tell the compiler the relationships between data on the stack:

```Rust
foo(a, b); // we are referring to variable `a` and `b` in scope.
```

However, you can use any convention to describe relationships, not just order or naming.
In this library we have developed a safe convention that uses concrete types.
Each concrete type can have a "current" value,
which is accessible to all functions that knows about the type:

```Rust
let a = RefCell::new(a); // RefCell prevents multiple mutable references at run time.
let a_guard = a.set_current();

foo::<A>(b); // we are referring to the current value of type `A` and the variable `b` in scope.

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
        player_shoot();
    }
});

fn player_shoot() {
    Current::with_current(|player: &RefCell<Player>| {
        let player = player.borrow();
        Current::with_current(|gun: &RefCell<Gun>| {
            let mut gun = gun.borrow_mut();
            gun.shoot(player.aim);
        });
    });
}
```

This makes it easier to decouple data from each other in the application structure.

The major motivation for this library is to have a convention that works across libraries,
such that two people can share code by depending on the same library,
without knowing what the other person is doing.
This is important for building higher level game libraries.

