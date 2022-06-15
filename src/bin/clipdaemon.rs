use clipd::{clipboard, container};
use clipboard::{Clipboard, ClipboardType::X11};
use container::{Container, ContainerType::ClipdFs};

fn main() {
    println!("starting...");
    let cb = clipboard::create(X11);
    let mut ct = container::create(ClipdFs, "sys".to_string());
    loop {
        println!("waiting...");
        ct.add(None, cb.wait());
    }
}