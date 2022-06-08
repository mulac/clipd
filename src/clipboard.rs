pub trait Clipboard {
    fn paste(&self) -> String;
    fn copy(&self, value: String);
    // fn wait() -> String;
}

pub enum ClipboardType {
    X11
}

pub fn create(ctype: ClipboardType) -> impl Clipboard {
    match ctype {
        ClipboardType::X11 => {
            x11::new()
        }
    }
}

mod x11 {

pub fn new() -> x11_clipboard::Clipboard {
    x11_clipboard::Clipboard::new().expect("Cannot create new x11_clipboard")
}

impl super::Clipboard for x11_clipboard::Clipboard {
    fn paste(&self) -> String {
        let val = self.load(
            self.setter.atoms.clipboard,
            self.setter.atoms.utf8_string,
            self.setter.atoms.property,
            std::time::Duration::from_secs(3)
        ).unwrap();
        String::from_utf8(val)
            .expect("[x11_clipboard] Value fetched from clipboard cannot be converted to a String")
    }

    fn copy(&self, value: String) {
        self.store(
            self.setter.atoms.clipboard, 
            self.setter.atoms.utf8_string,
            value.as_bytes()
        ).unwrap_or_else(|err| panic!("[x11_clipboard] Cannot store '{}': {}", value, err))
    }
}
}