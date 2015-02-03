current [![Build Status](https://travis-ci.org/PistonDevelopers/current.svg)](https://travis-ci.org/PistonDevelopers/current)
=======

A library for setting current values for stack scope, such as application structure

Example project: [Sea Birds' Breakfast](https://github.com/bvssvni/ld31)

### How to use it

See [Best coding practices with current objects](https://github.com/PistonDevelopers/current/issues/15)

This also posts safety guidelines for the library.

### Motivation

In game programming, there are many kinds of "current" values:

* The current window
* The current device
* The current sound driver
* The current player object

There are two ways to use this library:

* An unsafe version that ease refactoring between current objects and mutable references
* A safe version that can be used in experimental library design

By setting these up as "current" values, you don't have to pass them around to each method.
For example, you can write code like this (demonstrating the unsafe version):

```Rust
e.press(|button| {
    let gun = unsafe { &mut *current_gun() };
    let player = unsafe { &mut *current_player() };
    if button == SHOOT {
        gun.shoot(player.aim);
    }
});
```

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)
