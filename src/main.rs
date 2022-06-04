mod cli;
mod clipboard;
use clipboard::Clipboard;
use structopt::StructOpt;

fn main() {
    let opt = cli::Opt::from_args();
    let mut cb = clipboard::clipd_fs::open(opt.container);
    match opt.cmd {
        cli::Command::Copy { key, value} => {
            cb.add(key, value);
        },

        cli::Command::Paste { key } => {
            println!("{}", cb.get(key).unwrap());
        }

        cli::Command::Clear => {
            // TODO: add a warning
            cb.clear();
        }
    };  
}