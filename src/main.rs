mod cli;
mod container;
mod clipboard;
use std::io::Read;
use container::{Container, ContainerType::ClipdFs};
use clipboard::{Clipboard, ClipboardType::X11};
use structopt::StructOpt;

fn fetch_value() -> String {
    if atty::is(atty::Stream::Stdin) {
        return clipboard::create(X11).paste()
    }
    
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).expect("couldn't read from stdin");
    buf
}

fn main() {
    let opt = cli::Opt::from_args();
    let mut cnt = container::create(ClipdFs, opt.container);

    match opt.cmd {
        cli::Command::Copy { key, value } => {
            cnt.add(key, value.unwrap_or_else(fetch_value));
        },
        cli::Command::Paste { key } => {
            println!("{}", cnt.get(key).unwrap());
        }
        cli::Command::Clear => { cnt.clear() } // TODO: add a warning
        cli::Command::Show => { println!("{}", cnt.show()) }
    };  
}