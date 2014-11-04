extern crate current;

use current::Current;
use std::cell::RefCell;

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

impl<T: TextProperty> current::Get<T> for Text {
    fn get(obj: &T) -> Text {
        Text(obj.get_text().to_string())
    }
}

impl<T: TextProperty> current::Modifier<T> for Text {
    fn modify(self, obj: &mut T) {
        let Text(text) = self;
        obj.set_text(text)
    }
}

fn print_text<T: TextProperty>() {
    // /*
    let Text(text) = current::get::<T, Text>();
    println!("{}", text);
    current::set::<T, Text>(Text("world!".to_string()));
    // */

    /*
    Current::with_current_unwrap(|val: &RefCell<T>| {
        let mut val = val.borrow_mut();
        println!("{}", val.get_text());
        val.set_text("world!".to_string());
    });
    */
}

fn bar() {
    let bar = RefCell::new(Foo { text: "good bye".to_string() });
    let guard = bar.set_current();
    print_text::<Foo>();
    print_text::<Foo>();
    drop(guard);
}

fn main() {
    let foo = RefCell::new(Foo { text: "hello".to_string() });
    {
        let guard = foo.set_current();
        print_text::<Foo>();
        print_text::<Foo>();
        bar();
        drop(guard);
    }
    foo.borrow_mut().text = "hi!".to_string();
}
