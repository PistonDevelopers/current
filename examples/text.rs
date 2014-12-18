extern crate current;

use current::{ Get, Set, Current, CurrentGuard };

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

fn print_text<T: 'static + TextProperty>() {
    let Text(text) = Current::<T>.get();
    println!("{}", text);
    Current::<T>.set(Text("world!".to_string()));
}

fn bar() {
    let mut bar = Foo { text: "good bye".to_string() };
    let guard = CurrentGuard::new(&mut bar);
    print_text::<Foo>();
    print_text::<Foo>();
    drop(guard);
}

fn main() {
    let mut foo = Foo { text: "hello".to_string() };
    {
        let guard = CurrentGuard::new(&mut foo);
        print_text::<Foo>();
        print_text::<Foo>();
        bar();
        drop(guard);
    }
    foo.text = "hi!".to_string();
}
