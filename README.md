# Current
A library for setting current values for stack scope, such as application structure

Current objects are put on a shadow stack for easy access by type.
The type is used as an identifier to get the latest current object in scope.
They are used as a better alternative in Rust to globals.

There are two objects in this library:

- `CurrentGuard` is used to create a current object using a mutable reference
- `Current` is used to access the reference by type

### How to use it

See [Best coding practices with current objects](https://github.com/PistonDevelopers/current/issues/15)
