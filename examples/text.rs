#![feature(phase)]

#[phase(plugin, link)] extern crate current;

use std::cell::RefCell;
use current::{ Get, Set, Current };

pub struct Foo {
    text: String
}

pub trait TextProperty {
    fn get_text(&self) -> &str;
    fn set_text(&mut self, text: String);
}

impl TextProperty for Foo {
    fn get_text(&self) -> &str {
        self.text.as_slice()
    }
    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

pub struct Text(pub String);

impl<T: TextProperty> current::Get<Text> for T {
    fn get(&self) -> Text {
        Text(self.get_text().to_string())
    }
}

impl<T: TextProperty> current::Modifier<T> for Text {
    fn modify(self, obj: &mut T) {
        let Text(text) = self;
        obj.set_text(text)
    }
}

fn print_text() {
    unsafe {
        let Text(text) = current_foo().get();
        println!("{}", text);
        current_foo().set(Text("world!".to_string()));
    }
}

fn bar() {
    let bar = Foo { text: "good bye".to_string() };
    let bar_2 = Foo { text: "good bye".to_string() };

    let bar = RefCell::new(bar);
    let bar_2 = RefCell::new(bar_2);
    current! {
        FOO: bar,
        FOO_2: bar_2
        || {
            print_text();
            print_text();
        }
    }

    /*
    let bar = RefCell::new(bar);
    let bar_2 = RefCell::new(bar_2);
    FOO.set(&bar, || {
        FOO_2.set(&bar_2, || {
            print_text();
            print_text();
        });
    });
    */
}

scoped_thread_local!(static FOO: RefCell<Foo>)
scoped_thread_local!(static FOO_2: RefCell<Foo>)

unsafe fn current_foo() -> Current<Foo> {
    Current::new(&FOO)
}

#[allow(dead_code)]
unsafe fn current_foo_2() -> Current<Foo> {
    Current::new(&FOO_2)
}

fn main() {
    let mut foo = Foo { text: "hello".to_string() };
    foo = {
        let foo = RefCell::new(foo);
        FOO.set(&foo, || {
            print_text();
            print_text();
            bar();
        });
        foo.into_inner()
    };
    foo.text = "hi!".to_string();
}
