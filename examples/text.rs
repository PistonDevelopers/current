extern crate current;

use current::Current;

pub struct Foo {
    text: String
}

pub trait Text {
    fn get_text(&self) -> &str;
    fn set_text(&mut self, text: String);
}

impl Text for Foo {
    fn get_text(&self) -> &str {
        self.text.as_slice()
    }
    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

fn print_text<T: Text>() {
    let scope = &mut ();
    let val: &mut T = Current::current_unwrap(scope);
    println!("{}", val.get_text());
    val.set_text("world!".to_string());
}

fn bar() {
    let mut bar = Foo { text: "good bye".to_string() };
    let guard = bar.set_current();
    print_text::<Foo>();
    print_text::<Foo>();
    drop(guard);
}

fn main() {
    let mut foo = Foo { text: "hello".to_string() };
    {
        let guard = foo.set_current();
        print_text::<Foo>();
        print_text::<Foo>();
        bar();
        drop(guard);
    }
    foo.text = "hi!".to_string();
}

