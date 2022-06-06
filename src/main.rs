mod cli;
mod container;
mod clipboard;
use container::{Container, ContainerType};
use clipboard::{Clipboard, ClipboardType};
use structopt::StructOpt;

fn main() {
    let opt = cli::Opt::from_args();
    let mut cnt = container::create(ContainerType::ClipdFs, opt.container);
    match opt.cmd {
        cli::Command::Copy { key, value} => {
            cnt.add(key, value.unwrap_or_else(|| clipboard::create(ClipboardType::X11).paste()));
        },
        cli::Command::Paste { key } => { println!("{}", cnt.get(key).unwrap()) }
        cli::Command::Clear => { cnt.clear() } // TODO: add a warning
        cli::Command::Show => { println!("{}", cnt.show()) }
    };  
}