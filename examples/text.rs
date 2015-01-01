extern crate current;

use current::{ Current, CurrentGuard };

pub struct Foo {
    text: String
}

fn print_foo() {
    let foo = unsafe { &*Current::<Foo>::new() };
    println!("{}", foo.text);
    unsafe { &mut *Current::<Foo>::new() }.text = "world!".to_string();
}

fn bar() {
    let mut bar = Foo { text: "good bye".to_string() };
    let guard = CurrentGuard::new(&mut bar);
    print_foo();
    print_foo();
    drop(guard);
}

fn main() {
    let mut foo = Foo { text: "hello".to_string() };
    {
        let guard = CurrentGuard::new(&mut foo);
        print_foo();
        print_foo();
        bar();
        drop(guard);
    }
    foo.text = "hi!".to_string();
}
