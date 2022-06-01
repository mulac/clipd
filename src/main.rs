mod cli;
mod clipboard;
use structopt::StructOpt;

fn main() {
    // TODO: FACTORY!
    let cb = clipboard::ClipdFS::new();
    let opt = cli::Opt::from_args();
    match opt.cmd {
        cli::Command::Copy { key, value} => {
            println!("Copying! Key: {:?}, Value: {:?}", key, value);
            // cb.add("key", "value");
            println!("{:?}", cb);
        },

        cli::Command::Paste { key } => {
            println!("Pasting {:?}!", key);
        }
    };  
}