// Run with:
// cargo run --example drop_order
//
// This demonstrates destruction order. Rust drops local variables in
// reverse declaration order: the last declared value is dropped first.

struct Tracked(&'static str);

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

struct Note<'a> {
    text: &'a str,
    _tracker: Tracked,
}

fn main() {
    println!("entering main");

    let text = String::from("hello");
    println!("created text");

    let note = Note {
        text: &text,
        _tracker: Tracked("note"),
    };
    println!("created note: {}", note.text);

    println!("leaving inner scope");
}
